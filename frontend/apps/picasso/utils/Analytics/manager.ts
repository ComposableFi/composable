import { GeneralEvent, PageViewEvent } from "@/utils/Analytics/type";
import * as path from "path";

type ProcessingEvent = PageViewEvent | GeneralEvent;

type Provider = {
  track: (event: GeneralEvent) => Promise<void>;
  getName: () => string;

  pageView: (event: PageViewEvent) => Promise<void>;
};

let events: Array<ProcessingEvent> = [];
let providers: Map<string, Provider> = new Map();
let shouldProcess: boolean = false;

const manager = {
  queue: (event: ProcessingEvent) => events.push(event),
  async eventProcessor(event: CustomEvent<ProcessingEvent>) {
    this.queue(event.detail);

    if (shouldProcess) {
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
    shouldProcess = state;
    this.execute();
  },
  addProvider(provider: Provider) {
    providers.set(provider.getName(), provider);
  },
  getProcessorState() {
    return shouldProcess;
  },
};

Object.freeze(manager);

export { manager };
