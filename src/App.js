import "regenerator-runtime/runtime";
import React from "react";
import { login, logout } from "./utils";

import "bootstrap/dist/css/bootstrap.min.css";
import "./global.css";

// Bootstrap Components
import { Container, Row, Nav, Navbar, Card } from "react-bootstrap";

import getConfig from "./config";
const { networkId } = getConfig(process.env.NODE_ENV || "development");

// image
import Logo from "./assets/jester_logo.svg";
import MoneyMemo from "./Components/MoneyMemo";
import Transactions from "./Components/Transaction";

import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";

export default function App() {
  return (
    <Router>
      <Navbar bg="#182722;" expand="lg">
        <Container>
          <Navbar.Brand href="/">
            <img className="logo" src={Logo}></img>
          </Navbar.Brand>
          <Navbar.Toggle aria-controls="basic-navbar-nav" />
          <Navbar.Collapse id="basic-navbar-nav">
            <Nav className="me-auto"></Nav>
            <Nav.Link href="/transactions">Transactions</Nav.Link>
            <Nav.Link
              onClick={window.walletConnection.isSignedIn() ? logout : login}
            >
              
              {window.walletConnection.isSignedIn()
                ? window.accountId
                : "Login"}
            </Nav.Link>
          </Navbar.Collapse>
        </Container>
      </Navbar>
      <Container>
        
        {window.walletConnection.isSignedIn() ? (
          <Row
            className="d-flex justify-content-center"
            style={{ marginTop: "10px" }}
          >
            <Switch>
              <Route exact path="/">
                <MoneyMemo />
              </Route>
              <Route exact path="/transactions">
                <Transactions />
              </Route>
            </Switch>
          </Row>
        ) : (
          <Row className="d-flex justify-content-center">
            <Card>
              <Card.Body>
                {" "}
                <Card.Title>Please Sign In to your wallet</Card.Title>
              </Card.Body>
            </Card>
          </Row>
        )}
      </Container>
    </Router>
  );
}
