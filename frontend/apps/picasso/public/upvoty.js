var script = document.createElement("script");
script.onload = function () {
  upvoty.init("render", {
    boardHash:
      "09abccedb1fb8876cced2a2d4c96f08823df77447bb7daafa10d24dbb2661464",
    ssoToken: null,
    baseUrl: "composable-fi.upvoty.com",
  });
};
document.head.appendChild(script);
script.src = "https://composable-fi.upvoty.com/javascript/upvoty.embed.js";
