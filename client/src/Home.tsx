import React from 'react';
import Grid from '@material-ui/core/Grid';
import Button from '@material-ui/core/Button';
import styles from './Home.module.css';

const Home: React.FC = () => {
    return (
        <Grid
            container
            direction="column"
            justify="center"
            alignItems="center"
            className={styles.grid}>
            <div className="Home">
                <Button variant="contained" color="primary" >
                    Login
                </Button>
            </div>
        </Grid>
    );
}

export default Home;