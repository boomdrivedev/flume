//! `flume host`, the host's resident (record 0082; core.13): the one
//! process of flume's that runs between its commands, started by the
//! host's service manager. It keeps watch over the host: each of its
//! settles is a watch's settle of the places where something is pending,
//! which writes the host's watch, so flume runs attended while it runs
//! (119, 208, 225; core.62). It learns of a change in a merge queue from
//! the one subscription it holds to the merge queue's own resident, which
//! follows the repositories of the places a change there can move, and
//! settles on each change it hears; where it heard none for a minute, its
//! bound, it settles once as a backstop.
//!
//! The core stays synchronous behind this tokio edge: a settle is built
//! and run on tokio's blocking pool, and the edge decides when to ask
//! again. The edge names no merge queue (core.1): what it subscribes to is
//! the profile's, through [`Subscribes`].

use flume_core::{Answer, Engine};
use flume_seams::Seconds;
use std::collections::BTreeSet;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{watch, Notify};
use tokio::time::{sleep, sleep_until, Instant};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// How long the resident waits after a change it heard before it
/// settles, so the changes that come with it make one settle: a merge
/// queue writes its records several times as it decides a submission.
pub const GATHER: Duration = Duration::from_secs(2);

/// A watch's settle as the resident asks it: the places where something
/// is pending settled as a watch's settle settles them, then the
/// repositories a change in their merge queue can move a place of.
pub trait Settles: Send + Sync {
    /// One watch's settle, and the repositories to follow after it; the
    /// words that kept it from being asked otherwise.
    fn settle(&self) -> impl Future<Output = Result<BTreeSet<String>, String>> + Send;
}

/// The one subscription the resident holds to a merge queue's resident
/// (one-subscription-per-plugin; residents-ask-each-other-by-socket).
pub trait Subscribes: Send + Sync + 'static {
    /// Holds one subscription until it ends or `stop` is cancelled,
    /// following the repositories `follow` holds, the newest set each time
    /// it changes, and notifying `heard` at each change heard in one of
    /// them; the words saying why it ended.
    fn hold(
        &self,
        follow: watch::Receiver<BTreeSet<String>>,
        heard: Arc<Notify>,
        stop: CancellationToken,
    ) -> impl Future<Output = String> + Send;
}

/// Where the resident says what it did, a line at a time.
pub type Say = Arc<dyn Fn(&str) + Send + Sync>;

/// Runs the resident until `stop` is cancelled: a settle at once, then
/// one each time `queue`'s subscription hears a change, the changes heard
/// within [`GATHER`] of the first and while a settle runs making one
/// settle (one-read-in-flight), and one `every` after the last settle
/// where none was heard. The subscription follows the repositories the
/// newest settle gave, and is held again `every` after it ended. It ends
/// once its subscription's task has (workers-end-with-their-owner).
pub async fn run<S: Settles, Q: Subscribes>(
    settles: &S,
    queue: Q,
    every: Duration,
    stop: CancellationToken,
    say: Say,
) {
    let tasks = TaskTracker::new();
    let (follow, following) = watch::channel(BTreeSet::new());
    let heard = Arc::new(Notify::new());
    {
        let heard = heard.clone();
        let stop = stop.clone();
        let say = say.clone();
        tasks.spawn(async move {
            loop {
                let words = queue
                    .hold(following.clone(), heard.clone(), stop.child_token())
                    .await;
                if stop.is_cancelled() {
                    return;
                }
                say(&format!(
                    "the merge queue's subscription ended: {words}; held again in {} seconds",
                    every.as_secs()
                ));
                tokio::select! {
                    _ = stop.cancelled() => return,
                    _ = sleep(every) => {}
                }
            }
        });
    }
    loop {
        match settles.settle().await {
            Ok(followed) => {
                follow.send_if_modified(|held| {
                    let moved = *held != followed;
                    *held = followed;
                    moved
                });
            }
            Err(words) => say(&format!("the watch's settle was not asked: {words}")),
        }
        let backstop = Instant::now() + every;
        tokio::select! {
            _ = stop.cancelled() => break,
            _ = sleep_until(backstop) => {}
            _ = heard.notified() => {
                tokio::select! {
                    _ = stop.cancelled() => break,
                    _ = sleep(GATHER) => {}
                }
            }
        }
    }
    tasks.close();
    tasks.wait().await;
}

/// The settles of the host's own core: each builds the core afresh, so
/// it reads the host's configuration as it stands, and runs a watch's
/// settle with `bound` on tokio's blocking pool, then reads the
/// repositories to follow (208, 225; core.45).
pub struct Watch {
    build: fn() -> Result<Engine, String>,
    bound: Seconds,
    say: Say,
}

impl Watch {
    pub fn new(build: fn() -> Result<Engine, String>, bound: Seconds, say: Say) -> Watch {
        Watch { build, bound, say }
    }
}

impl Settles for Watch {
    async fn settle(&self) -> Result<BTreeSet<String>, String> {
        let build = self.build;
        let bound = self.bound;
        let (answer, followed) = tokio::task::spawn_blocking(move || {
            let core = build()?;
            let answer = core.settle_every(bound);
            Ok::<_, String>((answer, core.followed()))
        })
        .await
        .map_err(|error| format!("the settle's task ended: {error}"))??;
        match answer {
            Answer::Settled { places, .. } if !places.is_empty() => (self.say)(&format!(
                "settled {} place(s) where something was pending",
                places.len()
            )),
            Answer::Settled { .. } => {}
            other => (self.say)(&format!("the watch's settle answered {}", other.to_json())),
        }
        Ok(followed)
    }
}

/// The file a resident holds its lock on, in flume's state folder, so a
/// host runs one `flume host` at once (one-resident-per-tool).
pub const LOCK: &str = "host.lock";

/// Takes the host's resident's lock in `state`, held while the file it
/// gives is open; the words naming why not where another holds it.
pub fn lock(state: &std::path::Path) -> Result<std::fs::File, String> {
    use std::os::fd::AsRawFd as _;
    std::fs::create_dir_all(state)
        .map_err(|error| format!("making flume's state folder {}: {error}", state.display()))?;
    let path = state.join(LOCK);
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(&path)
        .map_err(|error| format!("opening {}: {error}", path.display()))?;
    // SAFETY: flock on a descriptor this process holds open.
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(format!(
            "flume host already runs on this host, holding {}",
            path.display()
        ));
    }
    Ok(file)
}

/// `flume host`: the host's resident over its own core and the merge
/// queue's subscription, keeping watch with `every` as its bound until a
/// SIGTERM or a SIGINT, on a runtime of one thread, the core's calls on
/// its blocking pool (the-runtime-flavour-is-chosen-in-main-and-tests-
/// pause-tokios-clock); the words that kept it from running otherwise.
pub fn host<Q: Subscribes>(
    build: fn() -> Result<Engine, String>,
    queue: Q,
    every: Seconds,
) -> Result<(), String> {
    let _lock = lock(&crate::host::config::flume_state()?)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| format!("building flume host's runtime: {error}"))?;
    runtime.block_on(async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut terminate = signal(SignalKind::terminate())
            .map_err(|error| format!("listening for SIGTERM: {error}"))?;
        let mut interrupt = signal(SignalKind::interrupt())
            .map_err(|error| format!("listening for SIGINT: {error}"))?;
        let stop = CancellationToken::new();
        let say: Say = Arc::new(|words| eprintln!("flume host: {words}"));
        say(&format!(
            "flume {} keeps watch over the host, settling at each change its merge queue's subscription hears and at least every {every} seconds",
            crate::VERSION
        ));
        let stopping = stop.clone();
        let watch = Watch::new(build, every, say.clone());
        let running = run(
            &watch,
            queue,
            Duration::from_secs(every),
            stop,
            say.clone(),
        );
        tokio::pin!(running);
        tokio::select! {
            _ = &mut running => {}
            _ = terminate.recv() => { stopping.cancel(); running.await; }
            _ = interrupt.recv() => { stopping.cancel(); running.await; }
        }
        say("stopped");
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Settles that count themselves and give `followed`.
    struct Counted {
        at: Mutex<Vec<Instant>>,
        followed: Mutex<BTreeSet<String>>,
    }

    impl Counted {
        fn new(followed: &[&str]) -> Arc<Counted> {
            Arc::new(Counted {
                at: Mutex::new(vec![]),
                followed: Mutex::new(followed.iter().map(|r| r.to_string()).collect()),
            })
        }

        fn count(&self) -> usize {
            self.at.lock().unwrap().len()
        }
    }

    impl Settles for Arc<Counted> {
        async fn settle(&self) -> Result<BTreeSet<String>, String> {
            self.at.lock().unwrap().push(Instant::now());
            Ok(self.followed.lock().unwrap().clone())
        }
    }

    /// A subscription the test drives: each hold records what it follows
    /// and hands the test its `heard`, and ends at the test's word.
    #[derive(Clone, Default)]
    struct Driven {
        holds: Arc<Mutex<usize>>,
        follows: Arc<Mutex<Vec<BTreeSet<String>>>>,
        heard: Arc<Mutex<Option<Arc<Notify>>>>,
        end: Arc<Notify>,
    }

    impl Driven {
        fn hear(&self) {
            if let Some(heard) = self.heard.lock().unwrap().as_ref() {
                heard.notify_one();
            }
        }
    }

    impl Subscribes for Driven {
        async fn hold(
            &self,
            mut follow: watch::Receiver<BTreeSet<String>>,
            heard: Arc<Notify>,
            stop: CancellationToken,
        ) -> String {
            *self.holds.lock().unwrap() += 1;
            *self.heard.lock().unwrap() = Some(heard);
            loop {
                let wanted = follow.borrow_and_update().clone();
                self.follows.lock().unwrap().push(wanted);
                tokio::select! {
                    _ = stop.cancelled() => return "stopped".into(),
                    _ = self.end.notified() => return "the test ended it".into(),
                    changed = follow.changed() => if changed.is_err() {
                        return "its follow was dropped".into();
                    }
                }
            }
        }
    }

    fn quiet() -> Say {
        Arc::new(|_| {})
    }

    const MINUTE: Duration = Duration::from_secs(60);

    /// The resident run in the background over `settles` and `queue`, and
    /// the token that stops it.
    fn started(
        settles: Arc<Counted>,
        queue: Driven,
    ) -> (CancellationToken, tokio::task::JoinHandle<()>) {
        let stop = CancellationToken::new();
        let run_stop = stop.clone();
        let handle =
            tokio::spawn(async move { run(&settles, queue, MINUTE, run_stop, quiet()).await });
        (stop, handle)
    }

    #[tokio::test(start_paused = true)]
    async fn it_settles_at_once_and_then_each_minute_with_nothing_heard() {
        let settles = Counted::new(&[]);
        let (stop, handle) = started(settles.clone(), Driven::default());
        sleep(Duration::from_secs(1)).await;
        assert_eq!(settles.count(), 1, "the settle at once");
        sleep(MINUTE).await;
        assert_eq!(settles.count(), 2, "the backstop a minute after");
        sleep(MINUTE * 3).await;
        assert_eq!(settles.count(), 5, "one each minute");
        stop.cancel();
        handle.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn a_change_heard_settles_and_moves_the_backstop() {
        let settles = Counted::new(&["/repository"]);
        let queue = Driven::default();
        let (stop, handle) = started(settles.clone(), queue.clone());
        sleep(Duration::from_secs(30)).await;
        queue.hear();
        sleep(GATHER + Duration::from_secs(1)).await;
        assert_eq!(settles.count(), 2, "a settle for the change heard");
        // The backstop is a minute after that settle, not after the first.
        sleep(Duration::from_secs(40)).await;
        assert_eq!(
            settles.count(),
            2,
            "no backstop within a minute of a settle"
        );
        sleep(Duration::from_secs(20)).await;
        assert_eq!(
            settles.count(),
            3,
            "the backstop a minute after the last settle"
        );
        stop.cancel();
        handle.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn many_changes_heard_together_make_one_settle() {
        let settles = Counted::new(&["/repository"]);
        let queue = Driven::default();
        let (stop, handle) = started(settles.clone(), queue.clone());
        sleep(Duration::from_secs(10)).await;
        for _ in 0..20 {
            queue.hear();
        }
        sleep(Duration::from_secs(1)).await;
        queue.hear();
        sleep(GATHER * 2).await;
        assert_eq!(
            settles.count(),
            3,
            "one settle for the burst, one for the change after it began"
        );
        sleep(GATHER * 2).await;
        assert_eq!(settles.count(), 3, "no settle with nothing heard since");
        stop.cancel();
        handle.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn the_subscription_follows_what_the_newest_settle_gave() {
        let settles = Counted::new(&["/one"]);
        let queue = Driven::default();
        let (stop, handle) = started(settles.clone(), queue.clone());
        sleep(Duration::from_secs(1)).await;
        *settles.followed.lock().unwrap() = ["/one", "/two"].map(String::from).into();
        sleep(MINUTE).await;
        *settles.followed.lock().unwrap() = BTreeSet::new();
        sleep(MINUTE).await;
        // Whether the subscription first saw the set before the first
        // settle gave one is the scheduler's: it is read from the first
        // set given on.
        let follows: Vec<BTreeSet<String>> = queue
            .follows
            .lock()
            .unwrap()
            .iter()
            .skip_while(|set| set.is_empty())
            .cloned()
            .collect();
        let expected: Vec<BTreeSet<String>> = vec![
            ["/one"].map(String::from).into(),
            ["/one", "/two"].map(String::from).into(),
            BTreeSet::new(),
        ];
        assert_eq!(follows, expected);
        stop.cancel();
        handle.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn an_ended_subscription_is_held_again_a_minute_on_and_the_backstop_settles_meanwhile() {
        let settles = Counted::new(&["/repository"]);
        let queue = Driven::default();
        let (stop, handle) = started(settles.clone(), queue.clone());
        sleep(Duration::from_secs(1)).await;
        assert_eq!(*queue.holds.lock().unwrap(), 1);
        queue.end.notify_one();
        sleep(Duration::from_secs(30)).await;
        assert_eq!(
            *queue.holds.lock().unwrap(),
            1,
            "held again within its minute"
        );
        sleep(Duration::from_secs(31)).await;
        assert_eq!(*queue.holds.lock().unwrap(), 2, "held again a minute on");
        assert_eq!(
            settles.count(),
            2,
            "the backstop settled while none was held"
        );
        stop.cancel();
        handle.await.unwrap();
    }

    #[test]
    fn a_second_resident_on_the_host_is_refused() {
        let state =
            std::env::temp_dir().join(format!("flume-resident-lock-{}", std::process::id()));
        let held = lock(&state).expect("the first resident's lock");
        let words = lock(&state).expect_err("a second resident's lock");
        assert!(words.contains("flume host already runs"), "{words}");
        drop(held);
        lock(&state).expect("the lock once the first resident ended");
        let _ = std::fs::remove_dir_all(&state);
    }

    #[tokio::test(start_paused = true)]
    async fn a_stop_ends_the_resident_and_its_subscription() {
        let settles = Counted::new(&["/repository"]);
        let queue = Driven::default();
        let (stop, handle) = started(settles.clone(), queue.clone());
        sleep(Duration::from_secs(1)).await;
        stop.cancel();
        tokio::time::timeout(Duration::from_secs(1), handle)
            .await
            .expect("the resident ends at its stop")
            .unwrap();
        assert_eq!(
            *queue.holds.lock().unwrap(),
            1,
            "no subscription held after the stop"
        );
    }
}
