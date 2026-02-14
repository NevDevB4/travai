import { RefreshCw } from 'lucide-react'
import '../styles/LMStudioStatus.css'

interface LMStudioStatusProps {
  isConnected: boolean
  onRetry: () => void
}

export default function LMStudioStatus({ isConnected, onRetry }: LMStudioStatusProps) {
  return (
    <div className={`lm-studio-status ${isConnected ? 'connected' : 'disconnected'}`}>
      <div className="status-indicator">
        <div className={`status-dot ${isConnected ? 'online' : 'offline'}`}></div>
        <span>{isConnected ? 'Connected to LM Studio' : 'Disconnected'}</span>
      </div>
      <button onClick={onRetry} className="retry-btn" title="Retry connection">
        <RefreshCw size={16} />
      </button>
    </div>
  )
}
