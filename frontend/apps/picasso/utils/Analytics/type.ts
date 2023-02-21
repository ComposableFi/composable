export type PageViewEvent = {
  path: string;
};

export type GeneralEvent = {
  category: string;
  action: string;
  label?: string;
  value?: number;
  nonInteraction?: boolean;
};
