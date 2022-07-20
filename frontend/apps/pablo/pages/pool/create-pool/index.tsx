import type { NextPage } from "next";
import {
  Container, Typography,
} from "@mui/material";
import { useRouter } from "next/router";
import Default from "@/components/Templates/Default";
import { CreatePool } from "@/components/Organisms/pool/CreatePool";
import { Link } from "@/components";
import { useEffect } from "react";
import { useDispatch } from "react-redux";
import {useDotSamaContext} from "substrate-react";
import useStore from "@/store/useStore";

const CreatePoolHome: NextPage = () => {

  const dispatch = useDispatch();
  const router = useRouter();

  const {extensionStatus} = useDotSamaContext();
  const {ui: { isPolkadotModalOpen } } = useStore();

  useEffect(() => {
    extensionStatus !== "connected" && !isPolkadotModalOpen && router.push('/pool');
  }, [extensionStatus, isPolkadotModalOpen, router]);

  const breadcrumbs = [
    <Link key="pool" underline="none" color="primary" href="/pool">
      Pool
    </Link>,
    <Typography key="create-pool" color="text.primary">
      Create pool
    </Typography>,
  ];

  return (
    <Default breadcrumbs={breadcrumbs}>
      <Container maxWidth="lg">
        <CreatePool />
      </Container>
    </Default>
  );
};

export default CreatePoolHome;
