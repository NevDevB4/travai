import { Copy, Check } from 'lucide-react'
import { useState } from 'react'
import hljs from 'highlight.js'
import 'highlight.js/styles/atom-one-dark.css'
import '../styles/MessageBubble.css'

interface MessageBubbleProps {
  role: string
  content: string
  codeLanguage?: string
  messageType: string
}

export default function MessageBubble({
  role,
  content,
  codeLanguage,
  messageType,
}: MessageBubbleProps) {
  const [copied, setCopied] = useState(false)

  const handleCopy = () => {
    navigator.clipboard.writeText(content)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const isCode = messageType === 'code' || messageType === 'file-analysis' || messageType === 'debug' || messageType === 'optimization'

  const renderContent = () => {
    if (isCode && codeLanguage) {
      try {
        const highlighted = hljs.highlight(content, { language: codeLanguage }).value
        return (
          <div className="code-block">
            <div className="code-header">
              <span className="code-language">{codeLanguage}</span>
              <button onClick={handleCopy} className="copy-btn">
                {copied ? <Check size={16} /> : <Copy size={16} />}
              </button>
            </div>
            <pre>
              <code dangerouslySetInnerHTML={{ __html: highlighted }} />
            </pre>
          </div>
        )
      } catch (e) {
        return <pre className="code-block">{content}</pre>
      }
    }

    // Parse markdown-like formatting
    return (
      <div className="text-content">
        {content.split('\n\n').map((paragraph, i) => (
          <p key={i}>{paragraph}</p>
        ))}
      </div>
    )
  }

  return (
    <div className={`message-bubble ${role}`}>
      <div className="message-content">
        {renderContent()}
      </div>
    </div>
  )
}
