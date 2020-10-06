import React, {useEffect, useState} from 'react';
import {Badge, Button, Card, CardColumns, Container, Nav, Navbar, Row} from "react-bootstrap";
import {list} from "./services/recipes";
import {Col} from 'react-bootstrap';

function App() {
    const [recipes, setRecipes] = useState<Array<Recipe>>([]);
    useEffect(() => {
        list().then(response => setRecipes(response.data));
    }, [setRecipes]);

    return <div>
        <Navbar bg="light" expand="lg">
            <Container>
                <Navbar.Brand href="#home">Tryunes</Navbar.Brand>
                <Nav className="mr-auto">
                    <Nav.Link href="#home">Recipes</Nav.Link>
                    <Nav.Link href="#link">Ingredients</Nav.Link>
                </Nav>
            </Container>
        </Navbar>
        <Container>
            <CardColumns>
                {recipes.map(recipe =>
                    <Card>
                        <Card.Body>
                            <Card.Title>{recipe.name} <Badge
                                variant={recipe.price > 10 ? "warning" : "info"}>{recipe.price} €</Badge></Card.Title>
                            <Card.Text>{recipe.description}</Card.Text>
                        </Card.Body>
                        <Card.Img variant="top" src={recipe.image}/>
                        <Card.Footer>
                            <Row>
                                <Col>
                                    <small>
                                        <div><b>Temps de preparation</b></div>
                                        <div>{recipe.preparation_duration} min</div>
                                    </small>
                                </Col>
                                <Col>
                                    <small>
                                        <div><b>Temps de cuisson</b></div>
                                        <div>{recipe.cooking_duration} min</div>
                                    </small>
                                </Col>
                            </Row>
                        </Card.Footer>
                    </Card>)}
            </CardColumns>
        </Container>
    </div>
}

export default App;
