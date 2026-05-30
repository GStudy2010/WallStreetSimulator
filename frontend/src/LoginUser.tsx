import { Link, useNavigate } from "react-router-dom";
import { useState } from "react";
import "./LoginUser.css";
import Modal from "./Modal";

export default function LoginUser() {
  const navigate = useNavigate();

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const [show400modal, setShow400modal] = useState(false);
  const [show403modal, setShow403modal] = useState(false);
  const [show500modal, setShow500modal] = useState(false);
  const [showLoggedInmodal, setShowLoggedInmodal] = useState(false);

  const handleSubmit = async (
    e: React.FormEvent<HTMLFormElement>
  ) => {
    e.preventDefault();

    try {
      const response = await fetch("/api/loginuser", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          email,
          password,
        }),
      });

      const data = await response.json();

      console.log(data);

      if (response.status === 400) {
        setShow400modal(true);
        return;
      }
      if (response.status === 403) {
        setShow403modal(true);
        return;
      }
      if (response.status === 500) {
        setShow500modal(true);
        return;
      }
      if (!response.ok) {
        throw new Error(data.message);
      }
      setShowLoggedInmodal(true);

      navigate("/app");
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

          <h1> Login to your account </h1>

          <p>
            Please notice that you need to verify your email first to login.
          </p>
        </div>

        <form
          className="register-form"
          onSubmit={handleSubmit}
        >

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
          <button
            type="submit"
            className="create-btn"
          >
            Login to your Trading Account
          </button>
        </form>

        <div className="footer-links">
          <span> You don't have an account? Create one</span>

          <Link to="/app/createAccount">
            Create your account for free
          </Link>
        </div>
      </div>
        <Modal
          isOpen={show403modal}
          title="Email Verification Required"
          message="You must verify your email address before logging in. Please check your inbox and click the verification link."
          onClose={() => setShow403modal(false)}
        />
        <Modal
          isOpen={show400modal}
          title="Your request is invalid"
          message="Request your provided and it's data is invalid for server."
          onClose={() => setShow400modal(false)}
        />
        <Modal
          isOpen={show500modal}
          title="Internal server error"
          message="Server had an issue processing your request"
          onClose={() => setShow500modal(false)}
        />
        <Modal
          isOpen={showLoggedInmodal}
          title="Logged in to your account"
          message="You logged in, happy trading"
          onClose={() => setShowLoggedInmodal(false)}
        />
    </main>
  );
}

