import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Link from "next/link";

import Layout from '../components/layout';
import utilStyles from '../utils.module.css';
import { getSortedPostsData } from '../lib/posts';
import Date from '../components/date';

import { motion } from "framer-motion"


export async function getStaticProps() {
  const allPostsData = getSortedPostsData();
  return {
    props: {
      allPostsData,
    },
  };
}


function App({ allPostsData }) {
  const [devicesMsg, setDevicesMsg] = useState("");

  async function get_all_devices_to_view() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setDevicesMsg(await invoke("get_all_devices_to_view"));
  }


  useEffect(() => {
    try {
      get_all_devices_to_view();

    } catch (error) {
      console.error(error);
    }

  }, [])

  return (
    <Layout home>

      <motion.section className={`${utilStyles.headingMd} ${utilStyles.padding1px}`}
        initial={{ scale: 0.8, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        transition={{ delay: 0.5 }}
      >

        <div className="card">
          <h1>Device</h1>
          <code>
            {devicesMsg}
          </code>

        </div>
        <div className="card">
          <h1>Changelog</h1>
          <ul>

            {allPostsData.map(({ id, date, title }) => (
              <li className={`${utilStyles.listItem} ${utilStyles.padding1px}`} key={id}>
                <Link href={`/posts/${id}`}>
                  <a>{title} </a>
                </Link>
                <small className={utilStyles.lightText}>
                  <Date dateString={date} />
                </small>
                <br />

              </li>
            ))}
          </ul>
        </div>

      </motion.section>
    </Layout>
  );
}

export default App;
