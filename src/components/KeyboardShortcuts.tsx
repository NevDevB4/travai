import { useState } from 'react'
import { X, HelpCircle } from 'lucide-react'
import '../styles/KeyboardShortcuts.css'

export default function KeyboardShortcuts() {
  const [isOpen, setIsOpen] = useState(false)

  const shortcuts = [
    { key: 'Ctrl+Shift+T', action: 'Toggle Travai window' },
    { key: 'Ctrl+Enter', action: 'Send message' },
    { key: 'Escape', action: 'Close dialog' },
    { key: 'Ctrl+N', action: 'New conversation' },
    { key: 'Ctrl+L', action: 'Focus message input' },
  ]

  return (
    <>
      <button
        onClick={() => setIsOpen(true)}
        className="shortcuts-toggle"
        title="Keyboard shortcuts"
      >
        <HelpCircle size={18} />
      </button>

      {isOpen && (
        <div className="shortcuts-modal">
          <div className="shortcuts-content">
            <div className="shortcuts-header">
              <h2>Keyboard Shortcuts</h2>
              <button
                onClick={() => setIsOpen(false)}
                className="close-btn"
              >
                <X size={20} />
              </button>
            </div>

            <div className="shortcuts-list">
              {shortcuts.map((shortcut, i) => (
                <div key={i} className="shortcut-item">
                  <kbd className="shortcut-key">{shortcut.key}</kbd>
                  <span className="shortcut-action">{shortcut.action}</span>
                </div>
              ))}
            </div>

            <div className="shortcuts-footer">
              <p>Press <kbd>Ctrl+Shift+T</kbd> anywhere to activate Travai</p>
            </div>
          </div>
        </div>
      )}
    </>
  )
}
