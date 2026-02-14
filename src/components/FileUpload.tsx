import { useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { toast } from 'sonner'
import { Upload, X, FileCode } from 'lucide-react'
import '../styles/FileUpload.css'

interface FileUploadProps {
  conversationId: string
  onFileUploaded?: () => void
}

const SUPPORTED_EXTENSIONS = [
  '.js', '.jsx', '.ts', '.tsx', '.py', '.java', '.cpp', '.c', '.h', '.hpp',
  '.go', '.rs', '.rb', '.php', '.cs', '.swift', '.kt', '.scala', '.r', '.sql',
  '.html', '.css', '.json', '.xml', '.yaml', '.yml', '.toml', '.ini'
]

const EXTENSION_TO_LANGUAGE: Record<string, string> = {
  '.js': 'javascript', '.jsx': 'javascript', '.ts': 'typescript', '.tsx': 'typescript',
  '.py': 'python', '.java': 'java', '.cpp': 'cpp', '.c': 'c', '.h': 'c', '.hpp': 'cpp',
  '.go': 'go', '.rs': 'rust', '.rb': 'ruby', '.php': 'php', '.cs': 'csharp',
  '.swift': 'swift', '.kt': 'kotlin', '.scala': 'scala', '.r': 'r', '.sql': 'sql',
  '.html': 'html', '.css': 'css', '.json': 'json', '.xml': 'xml',
  '.yaml': 'yaml', '.yml': 'yaml', '.toml': 'toml', '.ini': 'ini',
}

export default function FileUpload({ conversationId, onFileUploaded }: FileUploadProps) {
  const [isDragging, setIsDragging] = useState(false)
  const [selectedFile, setSelectedFile] = useState<File | null>(null)
  const [question, setQuestion] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const getLanguageFromFileName = (fileName: string): string => {
    const ext = fileName.substring(fileName.lastIndexOf('.')).toLowerCase()
    return EXTENSION_TO_LANGUAGE[ext] || 'plaintext'
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(true)
  }

  const handleDragLeave = () => {
    setIsDragging(false)
  }

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(false)

    const files = e.dataTransfer.files
    if (files.length > 0) {
      const file = files[0]
      const ext = file.name.substring(file.name.lastIndexOf('.')).toLowerCase()

      if (!SUPPORTED_EXTENSIONS.includes(ext)) {
        toast.error(`Unsupported file type: ${ext}`)
        return
      }

      if (file.size > 5 * 1024 * 1024) {
        toast.error('File size must be less than 5MB')
        return
      }

      setSelectedFile(file)
    }
  }

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.currentTarget.files
    if (files && files.length > 0) {
      const file = files[0]
      const ext = file.name.substring(file.name.lastIndexOf('.')).toLowerCase()

      if (!SUPPORTED_EXTENSIONS.includes(ext)) {
        toast.error(`Unsupported file type: ${ext}`)
        return
      }

      if (file.size > 5 * 1024 * 1024) {
        toast.error('File size must be less than 5MB')
        return
      }

      setSelectedFile(file)
    }
  }

  const handleUpload = async () => {
    if (!selectedFile) {
      toast.error('Please select a file')
      return
    }

    setIsLoading(true)
    try {
      const fileContent = await selectedFile.text()
      const language = getLanguageFromFileName(selectedFile.name)

      await invoke('upload_and_analyze_file', {
        conversationId,
        fileName: selectedFile.name,
        fileType: language,
        fileContent,
        question: question || null,
      })

      setSelectedFile(null)
      setQuestion('')
      if (fileInputRef.current) {
        fileInputRef.current.value = ''
      }
      toast.success('File uploaded and analyzed')
      onFileUploaded?.()
    } catch (error) {
      toast.error('Failed to upload file')
      console.error(error)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="file-upload">
      <div
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        className={`drop-zone ${isDragging ? 'dragging' : ''}`}
      >
        <input
          ref={fileInputRef}
          type="file"
          onChange={handleFileSelect}
          accept={SUPPORTED_EXTENSIONS.join(',')}
          className="hidden"
        />

        {selectedFile ? (
          <div className="file-selected">
            <FileCode size={32} />
            <div>
              <p className="file-name">{selectedFile.name}</p>
              <p className="file-size">{(selectedFile.size / 1024).toFixed(2)} KB</p>
            </div>
            <button
              onClick={() => {
                setSelectedFile(null)
                if (fileInputRef.current) {
                  fileInputRef.current.value = ''
                }
              }}
              className="remove-btn"
            >
              <X size={20} />
            </button>
          </div>
        ) : (
          <div onClick={() => fileInputRef.current?.click()}>
            <Upload size={32} />
            <p className="drop-text">Drop file here or click to select</p>
            <p className="drop-hint">Supported: {SUPPORTED_EXTENSIONS.slice(0, 5).join(', ')}, and more</p>
          </div>
        )}
      </div>

      {selectedFile && (
        <div className="question-input">
          <textarea
            value={question}
            onChange={e => setQuestion(e.target.value)}
            placeholder="Ask a question about the code (optional)..."
            rows={2}
            disabled={isLoading}
          />
          <button
            onClick={handleUpload}
            disabled={isLoading}
            className="analyze-btn"
          >
            {isLoading ? 'ANALYZING...' : 'ANALYZE FILE'}
          </button>
        </div>
      )}
    </div>
  )
}
