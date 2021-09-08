/**
 * @file ICChatBot
 */

import React, { useCallback, useEffect, useState } from 'react';
import parseHtml from 'html-react-parser';
import '@progress/kendo-theme-material/dist/all.css';
import { Button } from '@progress/kendo-react-buttons';
import { Chat } from '@progress/kendo-react-conversational-ui';
import { ic_chatbot } from '../../../../declarations/ic_chatbot';
import FaqCard from '../FaqCard/FaqCard';

// TODO: Add favicon!!!

// The User instance of the bot.
const bot = {
  id: 0,
  avatarUrl: 'assets/astro.png'
};

// The User instance of the local user.
const user = {
  id: 1
};

/**
 * Component that displays a chat bot.
 */
const ICChatBot = () => {
  const [displayFaq, setDisplayFaq] = useState(false);
  const [faqBody, setFaqBody] = useState('');
  const [faqTitle, setFaqTitle] = useState('');
  const [messages, setMessages] = useState([]);
  const [sessionId, setSessionId] = useState('');

  // Initialize session.
  useEffect(async () => {
    try {
      const responseString = await ic_chatbot.init_session();
      const response = JSON.parse(responseString);

      // Set the session ID.
      setSessionId(response[0].session_id);

      // Process each message block in the response.
      if (response.length > 0)
        processMessageBlocks(response[1]);
    }
    catch {
      // Add an error message.
      // TODO: Have a function to add a basic text message such as this!!!
      const botMessage = {
        author: bot,
        text: 'Sorry, couldn\'t establish communication with mission control. Please try again later.',
        timestamp: new Date()
      };
      setMessages(prevMessages => [...prevMessages, botMessage]);
    }
  }, []);

  // Callback fired when a FAQ card is clicked.
  const onFaqCardClick = useCallback(
    (title, body) => {
      setFaqTitle(title);
      setFaqBody(body);
      setDisplayFaq(true);
    },
    []
  );

  // Callback fired when FAQ back button is clicked.
  const onFaqBackButtonClick = useCallback(
    () => {
      setDisplayFaq(false);
    },
    []
  );

  // Fires when the user types a message and clicks the Send button or presses Enter.
  const addNewMessage = useCallback(
    async (event) => {
      // Add the user message.
      setMessages(prevMessages => [...prevMessages, event.message]);

      // TODO: Wait for init_session to set sessionId before we allow a question!!!
      // TODO: Add a typing animation for the bot message (use Message.typing)!!!
      try {
        // Send the user message to the ic_chatbot canister and get the response.
        const responseString = await ic_chatbot.get_next_block(sessionId, event.message.text);
        const messageBlocks = JSON.parse(responseString);

        // Process each message block in the response.
        processMessageBlocks(messageBlocks);
      }
      catch {
        // Add an error message.
        const botMessage = {
          author: bot,
          text: 'Sorry, something went wrong. Please ask another question.',
          timestamp: new Date()
        };
        setMessages(prevMessages => [...prevMessages, botMessage]);
      }
    },
    [sessionId]
  );

  // Process each message block in the specified messageBlocks array.
  const processMessageBlocks = (messageBlocks) => {
    messageBlocks.forEach(messageBlock => {
      // Create the bot message based on the component_type.
      const botMessage = {
        author: bot,
        text: messageBlock.text ? messageBlock.text : undefined,
        timestamp: new Date()
      };
      switch (messageBlock.component_type) {
        case 'button':
          const attachments = [];
          messageBlock.buttons.forEach((button, index, buttons) => {
            const body = parseHtml(button.payload);
            attachments.push({
              title: button.title,
              body: body,
              handleClick: () => onFaqCardClick(button.title, body),
              isFirst: index == 0,
              isLast: index === buttons.length - 1
            });
          });
          botMessage.attachmentLayout = 'list';
          botMessage.attachments = attachments;
          break;
        case 'quick_replies':
          const suggestedActions = [];
          response.quick_replies.forEach(quickReply => {
            suggestedActions.push({
              type: 'reply',
              value: quickReply.title
            });
          });
          botMessage.suggestedActions = suggestedActions;
          break;
        case 'text':
          break;
        default:
          // Ignore jump message blocks and unrecognized message blocks.
          return;
      }

      // Add the bot message.
      setMessages(prevMessages => [...prevMessages, botMessage]);
    });
  };

  return (
    <div className='page'>
      <div className='container'>
        <div className='app-bar'>
          <div className='app-bar__left'>
            {displayFaq &&
              <Button
                className='app-bar__button'
                type='button'
                look='flat'
                onClick={onFaqBackButtonClick}
              >
                <span className='material-icons-outlined app-bar__button__icon'>arrow_back</span>
              </Button>
            }
          </div>
          <div>
            {displayFaq ? 'Support' : 'DFINITY'}
          </div>
          <div className='app-bar__right'></div>
        </div>
        {displayFaq &&
          <div className='chat-overlay k-chat'>
            <div className='chat-overlay__content'>
              <div className='chat-overlay__content__title'>
                {faqTitle}
              </div>
              <div className='chat-overlay__content__body'>
                {faqBody}
              </div>
            </div>
          </div>
        }
        <Chat
          className='chat'
          attachmentTemplate={props => <FaqCard {...props.item} />}
          messages={messages}
          onMessageSend={addNewMessage}
          placeholder={'Type your question here...'}
          user={user}
        />
      </div>
    </div>
  );
};

export default ICChatBot;
