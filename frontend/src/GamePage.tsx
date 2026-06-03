import { useParams } from "react-router-dom";
import './GamePage.css'
export default function GamePage() {
  const { roomId } = useParams();

  return (
    <main>
      <h1>Game {roomId}</h1>
    </main>
  );
}
