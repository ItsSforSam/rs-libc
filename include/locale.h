#ifndef _LOCALE_H
#define _LOCALE_H

struct lconv
{   
    /* The decimal point character used to format non monetary quantities */
    char* decimal_point; 
    /**
     * The character used to separate groups of digits to the left of the 
     *   decimal point character in formatted non monetary quantities.
     */
    char* thousands_sep;
    /** 
     * A string whose elements indicate the size of each group of digits
     * in formatted non monetary quantities.
     */
    char* grouping;
    /**
     * The currency symbol for the current locale
     * 
     * The first three characters contain the alphabetic international
     *  currency symbol in accordance with those specified in ISO 4217.
     * 
     * Codes for the representation of Currency and Funds.
     * 
     * The fourth character (preceding the null character) character used to separate the international currency symbol from
     *   the monetary quantity. 
     */
    char* int_curr_symbol;
    /* The local currency symbol applicable to the current locale. */
    char* currency_symbol;

    char* mon_decimal_point;
       /* The decimal point used to format monetary quantities.   */
    char* mon_thousands_sep;
        /**
         *  The separator for groups of digits to the left of the decimal point
         *   in formatted monetary quantities.
        */
    char* mon_grouping;
        /**
         *  A string whose elements indicate the size of each group of digits  
         *    in formatted monetary quantities. See below for more details.
        */
    char* positive_sign;
        /** The string used to indicate a non negative-valued formatted
         *   monetary quantity.
        */
    char* negative_sign;
        /**
         * The string used to indicate a negative-valued formatted monetary
         *  quantity.
         */
    char int_frac_digits;
        /* The number of fractional digits (those to the right of the         */
        /* decimal point) to be displayed in an internationally formatted     */
        /* monetary quantities.                                               */
    char frac_digits;
        /* The number of fractional digits (those to the right of the         */
        /* decimal point) to be displayed in a formatted monetary quantity.   */
    char p_cs_precedes;
        /* Set to 1 or 0 if the currency_symbol respectively precedes or      */
        /* succeeds the value for a non negative formatted monetary quantity. */
    char p_sep_by_space;
        /* Set to 1 or 0 if the currency_symbol respectively is or is not     */
        /* separated by a space from the value for a non negative formatted   */
        /* monetary quantity.                                                 */
    char n_cs_precedes;
        /* Set to 1 or 0 if the currency_symbol respectively precedes or      */
        /* succeeds the value for a negative formatted monetary quantity.     */
    char n_sep_by_space;
        /* Set to 1 or 0 if the currency_symbol respectively is or is not     */
        /* separated by a space from the value for a negative formatted       */
        /* monetary quantity.                                                 */
    char p_sign_posn;
        /* Set to a value indicating the position of the positive_sign for a  */
        /* non negative formatted monetary quantity. See below for more details*/
    char n_sign_posn;
        /* Set to a value indicating the position of the negative_sign for a  */
        /* negative formatted monetary quantity. */

};


#endif