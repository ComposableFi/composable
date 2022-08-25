import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components";
import {
  NotificationBox,
  useNotificationStore,
} from "@/components/Molecules/PersistentNotification";
import { Button } from "@mui/material";

const Notification = () => {
  const fire = useNotificationStore((state) => state.fire);
  const active = useNotificationStore((state) => state.active);

  const fireNotification = () => {
    fire({
      title: "This is a notification",
      subtitle: "Some action is happening on chain without a predefined timer.",
      variant: "default",
    });
  };

  const changeToError = () => {
    if (active) {
      fire({
        ...active,
        variant: "error",
      });
    }
  };

  const changeToSuccess = () => {
    if (active) {
      fire({
        ...active,
        variant: "success",
      });
    }
  };

  return (
    <Default>
      <PageTitle title={"Notification test runner"} />
      <Button onClick={fireNotification}>Fire notification</Button>
      <Button onClick={changeToSuccess}>Change to success</Button>
      <Button onClick={changeToError}>Change to error</Button>
      <NotificationBox />
    </Default>
  );
};

export default Notification;
