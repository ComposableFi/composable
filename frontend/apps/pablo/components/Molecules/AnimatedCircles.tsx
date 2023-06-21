import { keyframes } from "@mui/material";
import Box from "@mui/material/Box";
import * as React from "react";

const topGradientAnimation = keyframes`
  0% {
    background-position: -50vh -100vh;
  }

  50% {
    background-position: 50vh -100vh;
  }

  100% {
    background-position: -50vh -100vh;
  }
`;

const bottomGradientAnimation = keyframes`
  0% {
    background-position: 50vh 20vh;
  }
  50% {
    background-position: -70vh 20vh;
  }
  100% {
    background-position: 50vh 20vh;
  }
`;

const CIRCLE_SIZE = "100vw";
const BACKGROUND_ANIMATION_DURATION = "60s";
const sharedAnimationProps = {
  opacity: 0.5,
  backgroundAttachment: "fixed",
  width: "100%",
  position: "fixed",
  height: "100%",
  zIndex: "-1",
  mixBlendMode: "screen",
  backgroundBlendMode: "screen",
  backgroundSize: `${CIRCLE_SIZE} ${CIRCLE_SIZE}`,
  backgroundRepeat: "no-repeat",
};

export const AnimatedCircles = () => {
  return (
    <>
      <Box
        sx={{
          backgroundImage:
            "radial-gradient(50% 49.67% at 50% 49.78%, rgba(0, 135, 249, 1) 0%, rgba(0, 135, 249, 0.98) 9%, rgba(0, 126, 232, 0.91) 22%, rgba(0, 111, 204, 0.8) 36%, rgba(0, 89, 165, 0.65) 52%, rgba(0, 62, 114, 0.45) 68%, rgba(0, 29, 54, 0.21) 86%, rgba(0, 0, 0, 0) 100%)",
          animation: `${topGradientAnimation} ${BACKGROUND_ANIMATION_DURATION} linear infinite`,
          ...sharedAnimationProps,
          backgroundPosition: "-50vh -100vh",
        }}
      ></Box>
      <Box
        sx={{
          backgroundImage:
            "radial-gradient(50% 50% at 50% 50%, #860038 0%, rgba(112, 0, 46, 0.84) 20%, rgba(57, 0, 23, 0.43) 61%, rgba(0, 0, 0, 0) 100%)",
          animation: `${bottomGradientAnimation} ${BACKGROUND_ANIMATION_DURATION} linear infinite`,
          backgroundPosition: "50vh 20vh",
          ...sharedAnimationProps,
        }}
      ></Box>
    </>
  );
};
