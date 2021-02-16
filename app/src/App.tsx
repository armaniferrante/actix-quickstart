import React from "react";
import "./App.css";
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";

function App() {
  return (
    <div className="App">
      <Routes />
    </div>
  );
}

function Layout(props: any) {
  return (
    <div>
      <div>
        <Link to="/">Home</Link>
        <Link to="/page1">Page 1</Link>
        <Link to="/page2">Page 2</Link>
      </div>
      <div>{props.children}</div>
    </div>
  );
}

function Routes() {
  return (
    <Router>
      <Switch>
        <Layout>
          <Route path="/">
            <Default />
          </Route>
          <Route path="/page1">
            <Page1 />
          </Route>
          <Route path="/page2">
            <Page2 />
          </Route>
        </Layout>
      </Switch>
    </Router>
  );
}

function Default() {
  return <div>This is the default page yay</div>;
}

function Page1() {
  return <div>This is page 1 yay!</div>;
}

function Page2() {
  return <div>This is page 2 yay!</div>;
}

export default App;
