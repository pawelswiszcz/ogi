
import styles from './layout.module.css';
import Link from 'next/link';


import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faArrowLeftLong } from '@fortawesome/free-solid-svg-icons'


export const siteTitle = 'ogi';



export default function Layout({ children, home }) {


    return (
        <div>
            <header>
                {/* 
               

                {home ? (
                    <>

                        <Image
                            priority
                            src={Avatar}
                            className={utilStyles.borderCircle}
                            height={108}
                            width={108}
                            alt={name}
                        />
                        <h1 className={utilStyles.heading2Xl}>{name} </h1>
                    </>
                ) : (
                    <>
                        <Link href="/">
                            <a>
                                <Image
                                    priority
                                    src={Avatar}
                                    className={utilStyles.borderCircle}
                                    height={108}
                                    width={108}
                                    alt={name}
                                />
                            </a>
                        </Link>
                        <h2 className={utilStyles.headingLg}>
                            <Link href="/">
                                <a className={utilStyles.colorInherit}>{name}</a>
                            </Link>
                        </h2>
                    </>
                )}
                 */}
            </header>

            <main>{children}</main>
            {!home && (
                <div >
                    <Link href="/">
                        <a><FontAwesomeIcon icon={faArrowLeftLong} /></a>
                    </Link>
                </div>
            )}
        </div>
    );
}