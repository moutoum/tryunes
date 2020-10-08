import React, {useCallback, useRef, useState} from "react";
import {Button, Overlay, Popover} from "react-bootstrap";
import IngredientCreationForm from "../components/IngredientCreationForm";

const IngredientsPage = () => {
    const [show, setShow] = useState(false);
    const [target, setTarget] = useState(null);
    const ref = useRef();
    const toggleOverlay = useCallback((e) => {
        if (!show) setTarget(e.target);
        setShow(!show);
    }, [show, setShow, setTarget])

    return (
        <div className="d-flex justify-content-end">
            <Button variant="outline-success" onClick={toggleOverlay}>Ajouter un ingrédient</Button>
            <Overlay show={show} target={target} placement="bottom" container={ref.current}>
                <Popover id="popover-contained">
                    <Popover.Content>
                        <IngredientCreationForm onCreate={(a) => {
                            console.log(a);
                        }} onCancel={() => {
                            setShow(!show);
                        }}/>
                    </Popover.Content>
                </Popover>
            </Overlay>
        </div>
    );
}

export default IngredientsPage;