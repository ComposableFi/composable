export type TrackData = {
  name: string;
  data: Map<string, PageViewEvent | GeneralEvent>;
  type: EventType;
};

export type EventType = "PageView" | "Event";

export type PageViewEvent = {
  path: string;
};

export type GeneralEvent = {
  name: string;
  category: string;
  action: string;
  value: string | number;
  nonInteraction: boolean;
};
