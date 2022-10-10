import { useState } from "react";
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
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }


  return (
    <Layout home>

      <motion.section className={`${utilStyles.headingMd} ${utilStyles.padding1px}`}
        initial={{ scale: 0.8, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        transition={{ delay: 0.5 }}
      >


        <ul className={utilStyles.list}>

          {allPostsData.map(({ id, date, title }) => (
            <li className={`${utilStyles.listItem} ${utilStyles.padding1px} card`} key={id}>

              {/*<br />
              {id}
              <br />
            */}
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
      </motion.section>
      {/*
      <div className="container">
        <h1>Welcome to Tauri!</h1>

        <motion.div
          initial={{ scale: 0.8, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          transition={{ delay: 0.5 }}
        >
        </motion.div>

        <div className="row">
          <span className="logos">
            <a href="https://nextjs.org" target="_blank">
              <Image
                width={144}
                height={144}
                src={nextLogo}
                className="logo next"
                alt="Next logo"
              />
            </a>
          </span>
          <span className="logos">
            <a href="https://tauri.app" target="_blank">
              <Image
                width={144}
                height={144}
                src={tauriLogo}
                className="logo tauri"
                alt="Tauri logo"
              />
            </a>
          </span>
          <span className="logos">
            <a href="https://reactjs.org" target="_blank">
              <Image
                width={144}
                height={144}
                src={reactLogo}
                className="logo react"
                alt="React logo"
              />
            </a>
          </span>
        </div>

        <p>Click on the Tauri, Next, and React logos to learn more.</p>

        <div className="row">
          <div>
            <input
              id="greet-input"
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder="Enter a name..."
            />
            <button type="button" onClick={() => greet()}>
              Greet
            </button>
          </div>
        </div>

        <p>{greetMsg}</p>
      </div>
       */}
    </Layout>
  );
}

export default App;
