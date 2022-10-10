import type { AppProps } from "next/app";
import { AnimateSharedLayout } from "framer-motion";

//import 'bootstrap/dist/css/bootstrap.min.css';
import "../style.css";
import "../App.css";

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <AnimateSharedLayout>
      <Component {...pageProps} />
    </AnimateSharedLayout>
  );
}
