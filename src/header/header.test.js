import { render, screen } from '@testing-library/react';
import Header from './Header';

describe('<Header/>', () => {

    test('copyright text rendered', () => {
        render(<Header />);
        const element = screen.getByText(/made/i);
        expect(element).toBeInTheDocument();
    });

})