import { useState, useEffect, useRef } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { toast } from 'sonner'
import { Send, Code2, BookOpen, Bug, Zap } from 'lucide-react'
import MessageBubble from './MessageBubble'
import FileUpload from './FileUpload'
import '../styles/ChatInterface.css'

interface Message {
  id: string
  conversation_id: string
  role: string
  content: string
  code_language?: string
  message_type: string
  created_at: string
}

interface ChatInterfaceProps {
  conversationId: string
  isConnected: boolean
}

const PROGRAMMING_LANGUAGES = [
  'javascript', 'python', 'typescript', 'java', 'cpp', 'c', 'go', 'rust',
  'ruby', 'php', 'csharp', 'swift', 'kotlin', 'scala', 'r', 'sql',
  'html', 'css', 'json', 'xml', 'yaml'
]

export default function ChatInterface({ conversationId, isConnected }: ChatInterfaceProps) {
  const [messages, setMessages] = useState<Message[]>([])
  const [inputValue, setInputValue] = useState('')
  const [selectedLanguage, setSelectedLanguage] = useState('javascript')
  const [messageType, setMessageType] = useState<'text' | 'code' | 'explanation' | 'debug' | 'optimization'>('text')
  const [isLoading, setIsLoading] = useState(false)
  const [showFileUpload, setShowFileUpload] = useState(false)
  const messagesEndRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    loadMessages()
  }, [conversationId])

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  const loadMessages = async () => {
    try {
      const msgs: Message[] = await invoke('get_messages', { conversationId })
      setMessages(msgs)
    } catch (error) {
      toast.error('Failed to load messages')
    }
  }

  const handleSendMessage = async () => {
    if (!inputValue.trim() || !isConnected) {
      if (!isConnected) toast.error('Not connected to LM Studio')
      return
    }

    setIsLoading(true)
    try {
      let result

      switch (messageType) {
        case 'code':
          result = await invoke('generate_code', {
            conversationId,
            description: inputValue,
            language: selectedLanguage,
          })
          break
        case 'explanation':
          result = await invoke('explain_code', {
            conversationId,
            code: inputValue,
            language: selectedLanguage,
          })
          break
        case 'debug':
          result = await invoke('debug_code', {
            conversationId,
            code: inputValue,
            language: selectedLanguage,
          })
          break
        case 'optimization':
          result = await invoke('optimize_code', {
            conversationId,
            code: inputValue,
            language: selectedLanguage,
          })
          break
        default:
          result = await invoke('send_message', {
            conversationId,
            message: inputValue,
          })
      }

      setMessages(prev => [
        ...prev,
        result.user_message,
        result.assistant_message,
      ])
      setInputValue('')
      setMessageType('text')
    } catch (error) {
      toast.error('Failed to send message')
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="chat-interface">
      <div className="messages-container">
        {messages.length === 0 ? (
          <div className="empty-chat">
            <h2>Start a Conversation</h2>
            <p>Ask me to generate, explain, debug, or optimize code</p>
          </div>
        ) : (
          <>
            {messages.map(msg => (
              <MessageBubble
                key={msg.id}
                role={msg.role}
                content={msg.content}
                codeLanguage={msg.code_language}
                messageType={msg.message_type}
              />
            ))}
            <div ref={messagesEndRef} />
          </>
        )}
      </div>

      <div className="input-area">
        <div className="language-selector">
          <label>Language:</label>
          <select
            value={selectedLanguage}
            onChange={e => setSelectedLanguage(e.target.value)}
            disabled={isLoading}
          >
            {PROGRAMMING_LANGUAGES.map(lang => (
              <option key={lang} value={lang}>{lang.toUpperCase()}</option>
            ))}
          </select>
        </div>

        <div className="message-type-buttons">
          <button
            onClick={() => setMessageType('text')}
            className={messageType === 'text' ? 'active' : ''}
            disabled={isLoading}
          >
            Chat
          </button>
          <button
            onClick={() => setMessageType('code')}
            className={messageType === 'code' ? 'active' : ''}
            disabled={isLoading}
          >
            <Code2 size={16} /> Generate
          </button>
          <button
            onClick={() => setMessageType('explanation')}
            className={messageType === 'explanation' ? 'active' : ''}
            disabled={isLoading}
          >
            <BookOpen size={16} /> Explain
          </button>
          <button
            onClick={() => setMessageType('debug')}
            className={messageType === 'debug' ? 'active' : ''}
            disabled={isLoading}
          >
            <Bug size={16} /> Debug
          </button>
          <button
            onClick={() => setMessageType('optimization')}
            className={messageType === 'optimization' ? 'active' : ''}
            disabled={isLoading}
          >
            <Zap size={16} /> Optimize
          </button>
        </div>

        {showFileUpload && (
          <div className="file-upload-section">
            <FileUpload
              conversationId={conversationId}
              onFileUploaded={() => {
                setShowFileUpload(false)
                loadMessages()
              }}
            />
          </div>
        )}

        <div className="input-row">
          <textarea
            value={inputValue}
            onChange={e => setInputValue(e.target.value)}
            onKeyDown={e => {
              if (e.key === 'Enter' && e.ctrlKey) {
                handleSendMessage()
              }
            }}
            placeholder={
              messageType === 'code' ? 'Describe the code to generate...' :
              messageType === 'explanation' ? 'Paste code to explain...' :
              messageType === 'debug' ? 'Paste code to debug...' :
              messageType === 'optimization' ? 'Paste code to optimize...' :
              'Ask me anything about code...'
            }
            disabled={isLoading || !isConnected}
            rows={3}
          />
          <div className="input-actions">
            <button
              onClick={() => setShowFileUpload(!showFileUpload)}
              className="upload-btn"
              disabled={isLoading}
              title="Upload file"
            >
              📁
            </button>
            <button
              onClick={handleSendMessage}
              disabled={isLoading || !inputValue.trim() || !isConnected}
              className="send-btn"
            >
              {isLoading ? 'Thinking...' : <Send size={20} />}
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}
