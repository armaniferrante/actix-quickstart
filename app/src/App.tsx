import React from "react";
import { Provider } from 'react-redux';
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";
import { store } from './store';
import "./App.css";
import Login from './components/Login';
import Signup from './components/Signup';

function App() {
  return (
		<Provider store={store}>
			<Router>
				<Switch>
					<Layout>
						<Route exact path="/" component={Default} />
						<Route exact path="/login" component={LoginPage} />
						<Route exact path="/signup" component={SignupPage} />
						<Route exact path="/page1" component={Page1} />
						<Route exact path="/page2" component={Page2} />
					</Layout>
				</Switch>
			</Router>
		</Provider>
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

function Default() {
  return <div>This is the default page yay</div>;
}

function LoginPage() {
	return (
		<Login />
	);
}

function SignupPage() {
	return (
		<Signup />
	);
}


function Page1() {
  return <div>This is page 1 yay!</div>;
}

function Page2() {
  return <div>This is page 2 yay!</div>;
}

export default App;
