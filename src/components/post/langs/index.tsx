/**
 * @author: lencx
 * @create_at: Feb 01, 2020
 */

import React from 'react';
import { Link } from 'gatsby';

import styles from './lang.mod.scss';

export default function PostLangs({
  originURL = '',
  lang = '',
  defaultLang = '',
  dataSource = [],
}: {
  lang?: string;
  originURL?: string;
  defaultLang?: string;
  dataSource?: any;
}) {
  if (!dataSource.length) return null;
  // console.log(`[23] index.tsx: `, dataSource);
  return (
    <div className={styles.post_langs}>
      {lang === defaultLang ? (
        <b>Original</b>
      ) : (
        <Link to={originURL}>Original</Link>
      )}

      {dataSource.map((item: any) => {
        return (
          <span key={item.lang}>
            <b className={styles.symbol}>{' • '}</b>
            {lang === item.lang ? (
              <b>{item.langName}</b>
            ) : (
              <Link to={item.slug}>{item.langName}</Link>
            )}
          </span>
        );
      })}
    </div>
  );
}
