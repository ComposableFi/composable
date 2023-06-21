import dynamic from "next/dynamic";
import { FC, Fragment } from "react";
const NonSSRWrapper: FC = (props) => <Fragment>{props.children}</Fragment>;

export default dynamic(() => Promise.resolve(NonSSRWrapper), {
  ssr: false,
});
