import config from "@/constants/config";
import { GeneralEvent, PageViewEvent } from "@/utils/Analytics/type";
import { deleteCookie, hasCookie, setCookie } from "cookies-next";
import * as path from "path";
import { create } from "zustand";

type ProcessingEvent = PageViewEvent | GeneralEvent;

type Provider = {
  track: (event: GeneralEvent) => Promise<void>;
  getName: () => string;

  pageView: (event: PageViewEvent) => Promise<void>;
};

const COOKIES_ACCEPTED = "accepted_cookies";
const SECONDS_IN_6_MONTHS = 6 * 30 * 24 * 60 * 60;
let events: Array<ProcessingEvent> = [];
let providers: Map<string, Provider> = new Map();

export const useProcessorStateStore = create<{ shouldProcess: boolean }>()(
  (set) => ({
    shouldProcess: hasCookie(COOKIES_ACCEPTED),
  })
);

const manager = {
  queue: (event: ProcessingEvent) => events.push(event),
  async eventProcessor(event: CustomEvent<ProcessingEvent>) {
    this.queue(event.detail);

    if (useProcessorStateStore.getState().shouldProcess) {
      this.execute();
    }
  },
  listen() {
    document.addEventListener("TrackAnalytic", (event) =>
      this.eventProcessor(event)
    );
  },
  remove() {
    document.removeEventListener("TrackAnalytic", this.eventProcessor);
  },
  async execute() {
    await Promise.all(
      events.flatMap((event) =>
        Array.from(providers.entries()).map(([_, provider]) => {
          if ("path" in event) {
            return provider.pageView(event);
          }
          return provider.track(event);
        })
      )
    );
    this.flushQueue();
  },
  flushQueue() {
    events = [];
  },
  toggleProcessor(state: boolean) {
    useProcessorStateStore.setState(() => ({
      shouldProcess: state,
    }));

    // Set the cookie, otherwise remove it.
    if (state) {
      setCookie(COOKIES_ACCEPTED, true, {
        domain: config.analytics.allowedDomain,
        secure: true,
        maxAge: SECONDS_IN_6_MONTHS,
      });
    } else {
      deleteCookie(COOKIES_ACCEPTED);
    }

    this.execute();
  },
  addProvider(provider: Provider) {
    providers.set(provider.getName(), provider);
  },
};

Object.freeze(manager);

export { manager };
