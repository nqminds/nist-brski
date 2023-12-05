#!/bin/bash

echo ""
echo "Self Extracting Installer"
echo ""

export TMPDIR=`mktemp -d /tmp/selfextract.XXXXXX`

ARCHIVE=`awk '/^__ARCHIVE_BELOW__/ {print NR + 1; exit 0; }' $0`

tail -n+$ARCHIVE $0 | tar xzv -C $TMPDIR

CDIR=`pwd`
cd $TMPDIR
./installer

cd $CDIR
rm -rf $TMPDIR

exit 0

__ARCHIVE_BELOW__
�n�oe payload.tar �mo�0�y�O��I�ii5E[�Z��ڱ��I������Ŧ��~�	iiٺv�n�ߝ}y��ll'a"�⻽�0s[��U�{��߽}Ԡ��B=��P��������Zp;Ws�������iT0�븇��)�/F�G������p���V�Ӌ�񧓣�q�ѓ��x>ǣ�cY��2I�)������1�J�f$D�����k#�ݿ�<<��zi<���5����x���F����GO���������v(S
�2Rn-G����f�#�����\�����Y�w��+G�ؙ`�pz�m�Է�9Z��6 �CEZ,`B��k�lS��~-F����2%@����<KXG©*�)�w{��J���nH��*�� `.a=�aZ�#O`��A�%�dU��@��aVAB�1�zh#�B�<��n�Ǒ�s��3y˛PQ�7`CD��]���U��ǡ�8s�x��`S�;u�Ә��9fi���U�e\B�x��;N��b����=���b��S�����w�ڷ���_U��_����L��M��< II�H�����*��"ʋ��3��1��}@	!��ByV�6	]��'I�8S-:cEI�IY�$�BFB�*
�L#��D5 ǡW�F��:�g�?����L9��
��N����2�؃�
�>j}����9����:���$��D�i��c!xZbAꌫf����C��z�V���ѳ����}��F��`]�hG��f��}/T~��`0��`0�V�	IL (  