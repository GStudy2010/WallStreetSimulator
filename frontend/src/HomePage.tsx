import { useNavigate } from "react-router-dom";
import "./HomePage.css"

export default function HomePage() {
  const navigate = useNavigate();
  const handleLoginUser = () => {
    navigate("/app/loginUser");
  }
  const handleCreateAccount = () => {
    navigate("/app/createAccount");
  };
  return (
    <main className="home">
      <div className="grid-overlay" />

      <section className="hero">
        <span className="badge">Virtual Trading Platform</span>

        <h1>
          WALL STREET
          <br />
          SIMULATOR
        </h1>

        <p>
          Build your portfolio, compete against traders,
          and master the markets without risking real money.
        </p>

        <div className="hero-buttons">
          <button 
            className="primary-btn"
            onClick={handleCreateAccount}
          >
            Create account
          </button>

          <button 
          className="secondary-btn"
          onClick={handleLoginUser}
          >
            Login
          </button>
        </div>
      </section>
    </main>
  );
}
