import { useEffect, useState } from 'react';
import { AlertTriangle, Scale } from 'lucide-react';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion';
import { useTranslation } from 'react-i18next';

const STORAGE_KEY = 'disclaimer-important-open';

export function DisclaimerBanner() {
  const { t } = useTranslation();
  const [importantOpen, setImportantOpen] = useState(true);
  const [isHydrated, setIsHydrated] = useState(false);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    try {
      const stored = window.localStorage.getItem(STORAGE_KEY);
      if (stored) {
        setImportantOpen(stored === 'true');
      }
    } catch {
      setImportantOpen(true);
    }
    setIsHydrated(true);
  }, []);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    if (!isHydrated) return;
    window.localStorage.setItem(STORAGE_KEY, String(importantOpen));
  }, [importantOpen, isHydrated]);

  return (
    <Alert variant="warning" className="border-2">
      <Accordion
        type="single"
        collapsible
        value={importantOpen ? 'important' : ''}
        onValueChange={(value) => setImportantOpen(value === 'important')}
        className="w-full"
      >
        <AccordionItem value="important" className="border-0">
          <AccordionTrigger className="px-0 py-2 hover:no-underline">
            <div className="flex w-full items-center gap-3">
              <AlertTriangle className="h-5 w-5" />
              <AlertTitle className="text-base font-bold">
                {t('disclaimer.title')}
              </AlertTitle>
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <AlertDescription className="space-y-4 pt-2">
              <div className="space-y-2">
                <p className="text-sm font-semibold">
                  {t('disclaimer.liability.title')}
                </p>
                <p className="text-sm">{t('disclaimer.liability.description')}</p>
                <p className="text-sm font-semibold">
                  ⚠️ {t('disclaimer.liability.warning')}
                </p>
              </div>
              <div className="space-y-2 text-sm">
                <div className="flex items-center gap-2 font-semibold">
                  <Scale className="h-4 w-4" />
                  {t('disclaimer.license.title')}
                </div>
                <p>{t('disclaimer.license.description')}</p>
                <p className="font-semibold">{t('disclaimer.license.commercial')}</p>
                <p className="text-muted-foreground italic">
                  {t('disclaimer.license.footer')}
                </p>
              </div>
            </AlertDescription>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </Alert>
  );
}
