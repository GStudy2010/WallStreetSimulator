import './Modal.css'
type ModalProps = {
  isOpen: boolean;
  title: string;
  message: string;
  onClose: () => void;
};

export default function Modal({
  isOpen,
  title,
  message,
  onClose,
}: ModalProps) {
  if (!isOpen) return null;

  return (
    <div className="modal-overlay">
      <div className="modal">
        <h2>{title}</h2>

        <p>{message}</p>

        <button
          className="create-btn"
          onClick={onClose}
        >
          Close
        </button>
      </div>
    </div>
  );
}
