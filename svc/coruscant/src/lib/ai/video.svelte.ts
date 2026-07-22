import {
    createVideo as createVideoJob,
    getVideoJob,
} from "$lib/api/videos/videos.js";
import type { VideoRequest } from "$lib/api/models/index.js";

export type UseVideoOptions = {
    model?: string;
    onDone?: (video: string) => void;
    onError?: (error: string) => void;
};

export type StartVideoOptions = {
    /** Model override for this request. */
    model?: string | null;
    /** Aspect ratio (e.g. "16:9"). Omit for the model default. */
    aspectRatio?: string | null;
    /** Clip length in seconds. Omit (or pass `null`) for the default. */
    duration?: number | null;
};

/** How often (ms) to poll the job status endpoint while a render is running. */
const POLL_INTERVAL = 3000;

/**
 * Drives video generation as a background job: POST to create a job, then poll
 * its status until it reaches a terminal state. A completed job carries a
 * ready-to-play `url`; a failed job carries an error message.
 */
export function createVideo(options: UseVideoOptions = {}) {
    let video = $state<string | null>(null);
    let isLoading = $state(false);
    let isDone = $state(false);
    let error = $state<string | null>(null);

    // Bumped on every start/stop/reset so an in-flight poll loop from a previous
    // run can detect it has been superseded and bail out.
    let runId = 0;
    let timer: ReturnType<typeof setTimeout> | null = null;

    const onDone = options.onDone;
    const onError = options.onError;

    function clearTimer() {
        if (timer !== null) {
            clearTimeout(timer);
            timer = null;
        }
    }

    $effect(() => {
        return () => {
            runId += 1;
            clearTimer();
        };
    });

    function fail(message: string) {
        error = message;
        isLoading = false;
        isDone = true;
        onError?.(message);
    }

    function succeed(url: string) {
        video = url;
        isLoading = false;
        isDone = true;
        onDone?.(url);
    }

    async function poll(jobId: string, id: number) {
        if (id !== runId) return;
        try {
            const res = await getVideoJob(jobId);
            if (id !== runId) return;

            if (res.status !== 200) {
                fail(`The server responded with an error (${res.status}).`);
                return;
            }

            const job = res.data;
            switch (job.status) {
                case "completed": {
                    const url = job.video?.url ?? null;
                    if (url !== null) {
                        succeed(url);
                    } else {
                        fail("The video finished but no URL was returned.");
                    }
                    break;
                }
                case "failed":
                    fail(
                        job.error ??
                            "An error occurred while generating the video.",
                    );
                    break;
                default:
                    timer = setTimeout(() => poll(jobId, id), POLL_INTERVAL);
                    break;
            }
        } catch (err) {
            if (id !== runId) return;
            fail(
                err instanceof Error
                    ? err.message
                    : "An error occurred while checking the video status.",
            );
        }
    }

    async function start(prompt: string, startOptions: StartVideoOptions = {}) {
        runId += 1;
        const id = runId;
        clearTimer();
        video = null;
        isLoading = true;
        isDone = false;
        error = null;

        const body: VideoRequest = {
            prompt,
            model: startOptions.model ?? options.model ?? null,
            aspect_ratio: startOptions.aspectRatio ?? null,
            duration: startOptions.duration ?? null,
        };

        try {
            const res = await createVideoJob(body);
            if (id !== runId) return;
            if (res.status !== 202) {
                fail(`The server responded with an error (${res.status}).`);
                return;
            }
            void poll(res.data.id, id);
        } catch (err) {
            if (id !== runId) return;
            fail(
                err instanceof Error
                    ? err.message
                    : "An error occurred while starting the video.",
            );
        }
    }

    function stop() {
        runId += 1;
        clearTimer();
        isLoading = false;
        isDone = true;
    }

    function reset() {
        runId += 1;
        clearTimer();
        video = null;
        isLoading = false;
        isDone = false;
        error = null;
    }

    return {
        get video() {
            return video;
        },
        get isLoading() {
            return isLoading;
        },
        get isDone() {
            return isDone;
        },
        get error() {
            return error;
        },
        start,
        stop,
        reset,
    };
}
