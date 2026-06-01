import { useNavigate } from 'react-router-dom'
import './HomePage.css'
export default function GoBack() {
  const navigate = useNavigate();
  const handleMain = () => {
    navigate("/app");
  }
  return(
    <span className="badge" onClick={handleMain}>Virtual Trading Platform</span>
  )
}
