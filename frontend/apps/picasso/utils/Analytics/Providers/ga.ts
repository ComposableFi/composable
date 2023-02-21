import { GeneralEvent, PageViewEvent } from "@/utils/Analytics/type";
import ReactGA from "react-ga";

export function ga(token: string) {
  return {
    init() {
      ReactGA.initialize(token);
    },
    async track(event: GeneralEvent) {
      ReactGA.event(event);
    },

    async pageView(event: PageViewEvent) {
      ReactGA.pageview(event.path);
    },
    getName() {
      return "ga";
    },
  };
}
