import React, { useEffect, useState } from 'react';
import { render } from 'react-dom';
import { ic_chatbot } from "../../declarations/ic_chatbot";
import '@progress/kendo-theme-material/dist/all.css';
import { Chat } from "@progress/kendo-react-conversational-ui";

const bot = {
  id: 0
};

const user = {
  id: 1
};

const initialMessages = [
  {
    author: bot,
    timestamp: new Date(),
    text: "Hi! Welcome to DFINITY."
  },
  {
    author: bot,
    timestamp: new Date(),
    text: "Ask me a question and I'll find the answer for you."
  }
];

const ChatBot = () => {
  const [messages, setMessages] = useState(initialMessages);
  const [sessionId, setSessionId] = useState('');

  // Initialize session.
  useEffect(async () => {
    const responseString = await ic_chatbot.init_session();
    const response = JSON.parse(responseString);
    setSessionId(response.session_id);
  }, []);

  const addNewMessage = async (event) => {
    setMessages(prevMessages => [...prevMessages, event.message]);

    // TODO: Wait for init_session before we allow a question!!!
    const responseString = await ic_chatbot.get_next_block(sessionId, event.message.text);
    const response = JSON.parse(responseString);
    //console.log(response);//!!!
    const botResponseText = response[0].text;

    const botResponse = Object.assign({}, event.message);
    botResponse.author = bot;
    botResponse.text = botResponseText;
    setMessages(prevMessages => [...prevMessages, botResponse]);
  };

  return (
    <div>
      <Chat
        user={user}
        messages={messages}
        onMessageSend={addNewMessage}
        placeholder={"Type your question here..."}
        width={400}
      />
    </div>
  );
};

render(<ChatBot />, document.getElementById('app'));
