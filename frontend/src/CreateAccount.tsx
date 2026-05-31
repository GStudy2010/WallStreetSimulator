import { Link, useNavigate } from "react-router-dom";
import { useState } from "react";
import "./CreateAccount.css";
import Modal from "./Modal";

export default function CreateAccount() {
  const navigate = useNavigate();

  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  
  const [show400modal, setShow400modal] = useState(false);
  const [show500modal, setShow500modal] = useState(false);
  const [showAcountCreatedmodal, setShowAcountCreatedmodal] = useState(false);

  const handleSubmit = async (
    e: React.FormEvent<HTMLFormElement>
  ) => {
    e.preventDefault();

    if (password !== confirmPassword) {
      alert("Passwords don't match");
      return;
    }

    try {
      const response = await fetch("/api/createuser", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          name: username,
          email,
          password,
        }),
      });

      const data = await response.json();

      console.log(data);

      if (response.status === 400) {
        setShow400modal(true);
      }
      if (response.status === 500) {
        setShow500modal(true);
      }

      if (!response.ok) {
        throw new Error(data.message);
      }
      setShowAcountCreatedmodal(true);

      navigate("/app/loginUser");
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <main className="create-account-page">
      <div className="grid-overlay" />

      <div className="register-card">
        <div className="card-header">
          <span className="badge">
            WALL STREET SIMULATOR
          </span>

          <h1>Create Account</h1>

          <p>
            Start building your virtual portfolio and
            compete with traders around the world.
          </p>
        </div>

        <form
          className="register-form"
          onSubmit={handleSubmit}
        >
          <div className="input-group">
            <label>Username</label>
            <input
              type="text"
              placeholder="Enter username"
              value={username}
              onChange={(e) =>
                setUsername(e.target.value)
              }
            />
          </div>

          <div className="input-group">
            <label>Email</label>
            <input
              type="email"
              placeholder="Enter email"
              value={email}
              onChange={(e) =>
                setEmail(e.target.value)
              }
            />
          </div>

          <div className="input-group">
            <label>Password</label>
            <input
              type="password"
              placeholder="Enter password"
              value={password}
              onChange={(e) =>
                setPassword(e.target.value)
              }
            />
          </div>

          <div className="input-group">
            <label>Confirm Password</label>
            <input
              type="password"
              placeholder="Confirm password"
              value={confirmPassword}
              onChange={(e) =>
                setConfirmPassword(e.target.value)
              }
            />
          </div>

          <button
            type="submit"
            className="create-btn"
          >
            Create Trading Account
          </button>
        </form>

        <div className="footer-links">
          <span>Already have an account?</span>

          <Link to="/login">
            Login
          </Link>
        </div>
      </div>
        <Modal
          isOpen={show400modal}
          title="Email incorrect"
          message="Email you provided is incorrect"
          onClose={() => setShow400modal(false)}
        />
        <Modal
          isOpen={show500modal}
          title="Internal server error"
          message="Server had an issue processing your request"
          onClose={() => setShow500modal(false)}
        />
        <Modal
          isOpen={showAcountCreatedmodal}
          title="Logged in to your account"
          message="You logged in, happy trading"
          onClose={() => setShowAcountCreatedmodal(false)}
        />
    </main>
  );
}
