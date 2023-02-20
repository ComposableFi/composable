import { GeneralEvent, PageViewEvent } from "@/utils/Analytics/type";
import mixpanel from "mixpanel-browser";

export function mixPanel(token: string) {
  return {
    init() {
      mixpanel.init(token);
    },
    async track(event: GeneralEvent) {
      const { name, ...rest } = event;
      mixpanel.track(name, rest);
    },

    async pageView(event: PageViewEvent) {
      mixpanel.track("pageView", event);
    },

    getName() {
      return "mixpanel";
    },
  };
}
