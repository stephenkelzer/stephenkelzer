import { render, screen } from '@testing-library/react';
import Home from './home';

describe('<Home/>', () => {

    test('Main text rendered', () => {
        render(<Home />);
        const linkElement = screen.getByText('UNDER CONSTRUCTION');
        expect(linkElement).toBeInTheDocument();
    });

    test('Sub text renders', () => {
        render(<Home />);
        const linkElement = screen.getByText('New design scheduled to launch soon');
        expect(linkElement).toBeInTheDocument();
    });

});