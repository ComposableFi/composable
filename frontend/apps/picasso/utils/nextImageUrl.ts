const BASE_PATH = require("../app_config").BASE_PATH;

export function getImageURL(url: string) {
  return `${process.env.NODE_ENV === "production" ? BASE_PATH : ""}${url}`;
}
