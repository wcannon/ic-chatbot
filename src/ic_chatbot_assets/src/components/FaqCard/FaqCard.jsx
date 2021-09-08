/**
 * @file FaqCard
 */

import React from 'react';

/**
 * Component to display a FAQ card for Chat component attachmentTemplate prop.
 */
const FaqCard = attachment => {
  let className = 'faq-card';
  if (attachment.isFirst)
    className += ' faq-card--first';
  if (!attachment.isFirst && !attachment.isLast)
    className += ' faq-card--middle';
  if (attachment.isLast)
    className += ' faq-card--last';
  return (
    <div className={className} onClick={attachment.handleClick}>
      <div className='faq-card__header'>
        <span className='material-icons-outlined faq-card__header__icon'>
          article
        </span>
        <div className='faq-card__header__title'>{attachment.title}</div>
      </div>
      <div className='faq-card__body'>{attachment.body}</div>
    </div>
  );
};

export default FaqCard;
