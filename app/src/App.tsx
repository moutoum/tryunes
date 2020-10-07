import React, {useEffect, useState} from 'react';
import {CardColumns, Container, Nav, Navbar} from "react-bootstrap";
import {list} from "./services/recipes";
import RecipeCard from './components/RecipeCard';
import {BrowserRouter, Redirect, Route, Switch} from "react-router-dom";

function App() {
    const [recipes, setRecipes] = useState<Array<Recipe>>([]);
    useEffect(() => {
        list().then(response => setRecipes(response.data));
    }, [setRecipes]);

    return <div>
        <Navbar bg="light" expand="lg" sticky="top">
            <Container>
                <Navbar.Brand href="/">Tryunes</Navbar.Brand>
                <Nav className="mr-auto">
                    <Nav.Link href="/recipes">Recettes</Nav.Link>
                    <Nav.Link>Ingredients</Nav.Link>
                </Nav>
            </Container>
        </Navbar>
        <Container>
            <BrowserRouter>
                <Switch>
                    <Route exact path="/" render={() => <Redirect to="/recipes" />} />
                    <Route exact path="/recipes">
                        <CardColumns style={{marginTop: '1em'}}>
                            {recipes.map(recipe => <RecipeCard recipe={recipe}/>)}
                        </CardColumns>
                    </Route>
                </Switch>
            </BrowserRouter>

        </Container>
    </div>
}

export default App;
