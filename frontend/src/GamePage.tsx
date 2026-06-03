import { useParams } from "react-router-dom";
import './GamePage.css'
export default function GamePage() {
  const { gameId } = useParams();
  console.log("game id: ", gameId);
  return (
    <main>
      <h1>Game {gameId}</h1>
    </main>
  );
}
