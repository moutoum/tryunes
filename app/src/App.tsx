import React, {useState} from 'react';
import {Button, Container, Modal, Nav, Navbar} from "react-bootstrap";
import {BrowserRouter, Redirect, Route, Switch} from "react-router-dom";
import {APIProvider} from "./providers/APIProvider";
import RecipesPage from "./routes/RecipesPage";
import RecipeCreationForm from "./components/RecipeCreationForm";

function App() {
    const [show, setShow] = useState(false);

    return <div>
        <APIProvider>
            <Navbar bg="light" expand="lg" sticky="top">
                <Container>
                    <Navbar.Brand href="/">Tryunes</Navbar.Brand>
                    <Nav className="mr-auto">
                        <Nav.Link href="/recipes">Recettes</Nav.Link>
                    </Nav>
                    <Button variant="outline-success" onClick={() => setShow(true)}>Créer une recette</Button>
                    <Modal show={show} onHide={() => setShow(false)} centered size="lg">
                        <Modal.Header closeButton>
                            <Modal.Title>Création d'une recette</Modal.Title>
                        </Modal.Header>
                        <Modal.Body>
                            <RecipeCreationForm onRecipeCreated={() => setShow(false)}/>
                        </Modal.Body>
                    </Modal>
                </Container>
            </Navbar>
            <Container className="mt-2">
                <BrowserRouter>
                    <Switch>
                        <Route exact path="/" render={() => <Redirect to="/recipes"/>}/>
                        <Route exact path="/recipes" component={RecipesPage}/>
                    </Switch>
                </BrowserRouter>
            </Container>
        </APIProvider>
    </div>
}

export default App;
