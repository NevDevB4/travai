import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { toast, Toaster } from 'sonner'
import ChatInterface from './components/ChatInterface'
import ConversationList from './components/ConversationList'
import LMStudioStatus from './components/LMStudioStatus'
import KeyboardShortcuts from './components/KeyboardShortcuts'
import './App.css'

interface Conversation {
  id: string
  title: string
  created_at: string
  updated_at: string
}

function App() {
  const [conversations, setConversations] = useState<Conversation[]>([])
  const [activeConversation, setActiveConversation] = useState<string | null>(null)
  const [isConnected, setIsConnected] = useState(false)
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    checkConnection()
    loadConversations()
  }, [])

  const checkConnection = async () => {
    try {
      const connected: boolean = await invoke('check_lm_studio_connection')
      setIsConnected(connected)
      if (!connected) {
        toast.error('Cannot connect to LM Studio. Make sure it\'s running on localhost:1234')
      }
    } catch (error) {
      setIsConnected(false)
      toast.error('Failed to check LM Studio connection')
    }
  }

  const loadConversations = async () => {
    try {
      const convs: Conversation[] = await invoke('get_conversations')
      setConversations(convs)
      if (convs.length > 0 && !activeConversation) {
        setActiveConversation(convs[0].id)
      }
    } catch (error) {
      toast.error('Failed to load conversations')
    } finally {
      setIsLoading(false)
    }
  }

  const handleCreateConversation = async (title: string) => {
    try {
      const newConv: Conversation = await invoke('create_conversation', { title })
      setConversations([newConv, ...conversations])
      setActiveConversation(newConv.id)
      toast.success('Conversation created')
    } catch (error) {
      toast.error('Failed to create conversation')
    }
  }

  if (isLoading) {
    return (
      <div className="app-loading">
        <div className="loading-spinner"></div>
        <p>Loading Travai...</p>
      </div>
    )
  }

  return (
    <div className="app">
      <Toaster position="bottom-right" />
      <KeyboardShortcuts />
      
      <div className="app-header">
        <h1 className="app-title">TRAVAI</h1>
        <LMStudioStatus isConnected={isConnected} onRetry={checkConnection} />
      </div>

      <div className="app-container">
        <aside className="sidebar">
          <ConversationList
            conversations={conversations}
            activeConversation={activeConversation}
            onSelectConversation={setActiveConversation}
            onCreateConversation={handleCreateConversation}
          />
        </aside>

        <main className="main-content">
          {activeConversation ? (
            <ChatInterface
              conversationId={activeConversation}
              isConnected={isConnected}
            />
          ) : (
            <div className="empty-state">
              <h2>No Conversation Selected</h2>
              <p>Create a new conversation to get started</p>
            </div>
          )}
        </main>
      </div>
    </div>
  )
}

export default App
