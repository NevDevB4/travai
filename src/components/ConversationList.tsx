import { useState } from 'react'
import { Plus, Trash2 } from 'lucide-react'
import '../styles/ConversationList.css'

interface Conversation {
  id: string
  title: string
  created_at: string
  updated_at: string
}

interface ConversationListProps {
  conversations: Conversation[]
  activeConversation: string | null
  onSelectConversation: (id: string) => void
  onCreateConversation: (title: string) => void
}

export default function ConversationList({
  conversations,
  activeConversation,
  onSelectConversation,
  onCreateConversation,
}: ConversationListProps) {
  const [newTitle, setNewTitle] = useState('')
  const [isCreating, setIsCreating] = useState(false)

  const handleCreate = () => {
    if (newTitle.trim()) {
      onCreateConversation(newTitle)
      setNewTitle('')
      setIsCreating(false)
    }
  }

  return (
    <div className="conversation-list">
      <div className="list-header">
        <h2>Conversations</h2>
        <button
          onClick={() => setIsCreating(!isCreating)}
          className="new-btn"
          title="New conversation"
        >
          <Plus size={20} />
        </button>
      </div>

      {isCreating && (
        <div className="create-input">
          <input
            type="text"
            value={newTitle}
            onChange={e => setNewTitle(e.target.value)}
            onKeyDown={e => {
              if (e.key === 'Enter') handleCreate()
              if (e.key === 'Escape') {
                setIsCreating(false)
                setNewTitle('')
              }
            }}
            placeholder="Conversation title..."
            autoFocus
          />
          <button onClick={handleCreate} className="confirm-btn">Create</button>
        </div>
      )}

      <div className="conversations">
        {conversations.map(conv => (
          <div
            key={conv.id}
            className={`conversation-item ${activeConversation === conv.id ? 'active' : ''}`}
            onClick={() => onSelectConversation(conv.id)}
          >
            <div className="conv-title">{conv.title}</div>
            <div className="conv-date">
              {new Date(conv.updated_at).toLocaleDateString()}
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
