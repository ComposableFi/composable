import { NormalText } from "@/components/Organisms/Static/NormalText";
import { Link } from "@/components";

export const ContactUs = () => (
  <section>
    <NormalText>
      If you have any requests pursuant to the above provisions, questions or
      comments about this Privacy Policy, our data practices, or our compliance
      with applicable law, please contact us by email at:
      <Link variant="body3" href="mailto:legal@composable.finance">
        legal@composable.finance
      </Link>{" "}
      or by mail at: 1st Floor, The Sotheby Building, Rodney Village, Rodney
      Bay, LC 04 101, Gros Islet, Saint Lucia.
    </NormalText>
  </section>
);
