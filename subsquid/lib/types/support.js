"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.deprecateLatest = void 0;
let showLatestWarning = true;
function deprecateLatest() {
    if (showLatestWarning) {
        showLatestWarning = false;
        console.warn(`.isLatest, .asLatest properties are deprecated, if you believe this is a mistake, please leave a comment at https://github.com/subsquid/squid/issues/9`);
    }
}
exports.deprecateLatest = deprecateLatest;
//# sourceMappingURL=support.js.map