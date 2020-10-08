import React from 'react';
import {Container, Nav, Navbar} from "react-bootstrap";
import {BrowserRouter, Redirect, Route, Switch} from "react-router-dom";
import {APIProvider} from "./providers/APIProvider";
import RecipesPage from "./routes/RecipesPage";
import IngredientsPage from "./routes/IngredientsPage";

function App() {
    return <div>
        <APIProvider>
            <Navbar bg="light" expand="lg" sticky="top">
                <Container>
                    <Navbar.Brand href="/">Tryunes</Navbar.Brand>
                    <Nav className="mr-auto">
                        <Nav.Link href="/recipes">Recettes</Nav.Link>
                        <Nav.Link href="/ingredients">Ingredients</Nav.Link>
                    </Nav>
                </Container>
            </Navbar>
            <Container className="mt-3">
                <BrowserRouter>
                    <Switch>
                        <Route exact path="/" render={() => <Redirect to="/recipes"/>}/>
                        <Route exact path="/recipes" component={RecipesPage}/>
                        <Route exact path="/ingredients" component={IngredientsPage}/>
                    </Switch>
                </BrowserRouter>
            </Container>
        </APIProvider>
    </div>
}

export default App;
