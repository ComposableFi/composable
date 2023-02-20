import { GeneralEvent, PageViewEvent } from "@/utils/Analytics/type";

const ANALYTICS_EVENT_NAME = "TrackAnalytic";
const ANALYTICS_PAGEVIEW_EVENT_NAME = "PageViewAnalytic";

export function track(
  eventName: string,
  category: string,
  action: string,
  value: string | number,
  nonInteraction: boolean = false
) {
  const event = new CustomEvent<GeneralEvent>(ANALYTICS_EVENT_NAME, {
    detail: {
      name: eventName,
      category,
      action,
      value,
      nonInteraction,
    },
  });

  document.dispatchEvent(event);
}

export function pageView(path: string) {
  const event = new CustomEvent<PageViewEvent>(ANALYTICS_PAGEVIEW_EVENT_NAME, {
    detail: {
      path,
    },
  });

  document.dispatchEvent(event);
}
