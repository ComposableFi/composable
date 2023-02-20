import { GeneralEvent, PageViewEvent } from "@/utils/Analytics/type";
import ReactGA from "react-ga";

export function ga(token: string) {
  return {
    init() {
      ReactGA.initialize(token);
    },
    async track(event: GeneralEvent) {
      const payload = {
        category: event.category,
        action: event.action,
        label: event.name,
        nonInteraction: event.nonInteraction,
      };

      ReactGA.event(payload);
    },

    async pageView(event: PageViewEvent) {
      ReactGA.pageview(event.path);
    },
    getName() {
      return "ga";
    },
  };
}
