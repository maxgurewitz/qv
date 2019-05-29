import React from 'react';
import Grid from '@material-ui/core/Grid';
import Button from '@material-ui/core/Button';
import { login } from './auth';

const Home: React.FC = () => {
    return (
        <Grid
            container
            direction="column"
            justify="center"
            alignItems="center"
            className="centered-grid">
            <div className="Home">
                <Button onClick={login} variant="contained" color="primary" >
                    Login
                </Button>
            </div>
        </Grid>
    );
}

export default Home;