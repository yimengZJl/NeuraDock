import { useEffect } from 'react';
import { useForm } from 'react-hook-form';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { toast } from 'sonner';
import { useTranslation } from 'react-i18next';
import { Loader2, Info } from 'lucide-react';
import { cn } from '@/lib/utils';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';

export interface ProviderFormValues {
  name: string;
  domain: string;
  needs_waf_bypass: boolean;
  supports_check_in: boolean;
  check_in_bugged: boolean;
  // Optional fields
  login_path?: string;
  sign_in_path?: string;
  user_info_path?: string;
  token_api_path?: string;
  models_path?: string;
  api_user_key?: string;
}

interface ProviderDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  mode: 'create' | 'edit';
  providerId?: string;
  defaultValues?: Partial<ProviderFormValues>;
  onSubmit: (values: ProviderFormValues) => Promise<void>;
  isSubmitting?: boolean;
}

export function ProviderDialog({
  open,
  onOpenChange,
  mode,
  defaultValues,
  onSubmit,
  isSubmitting = false,
}: ProviderDialogProps) {
  const { t } = useTranslation();

  const {
    register,
    handleSubmit,
    formState: { errors },
    reset,
    watch,
    setValue,
  } = useForm<ProviderFormValues>({
    defaultValues: {
      name: '',
      domain: '',
      needs_waf_bypass: false,
      supports_check_in: true,
      check_in_bugged: false,
      login_path: '/login',
      sign_in_path: '/api/user/sign_in',
      user_info_path: '/api/user/self',
      token_api_path: '/api/token/',
      models_path: '/api/user/models',
      api_user_key: 'new-api-user',
    },
  });

  const needsWafBypass = watch('needs_waf_bypass');
  const supportsCheckIn = watch('supports_check_in');
  const checkInBugged = watch('check_in_bugged');

  useEffect(() => {
    if (open && defaultValues) {
      reset({
        name: defaultValues.name || '',
        domain: defaultValues.domain || '',
        needs_waf_bypass: defaultValues.needs_waf_bypass ?? false,
        supports_check_in: defaultValues.supports_check_in ?? true,
        check_in_bugged: defaultValues.check_in_bugged ?? false,
        login_path: defaultValues.login_path || '/login',
        sign_in_path: defaultValues.sign_in_path || '/api/user/sign_in',
        user_info_path: defaultValues.user_info_path || '/api/user/self',
        token_api_path: defaultValues.token_api_path || '/api/token/',
        models_path: defaultValues.models_path || '/api/user/models',
        api_user_key: defaultValues.api_user_key || 'new-api-user',
      });
    } else if (!open) {
      reset();
    }
  }, [open, defaultValues, reset]);

  const handleFormSubmit = async (values: ProviderFormValues) => {
    try {
      await onSubmit(values);
      onOpenChange(false);
    } catch (error) {
      console.error('Failed to save provider:', error);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>
            {mode === 'create' ? t('providerDialog.createTitle') : t('providerDialog.editTitle')}
          </DialogTitle>
          <DialogDescription>
            {mode === 'create'
              ? t('providerDialog.createDescription')
              : t('providerDialog.editDescription')}
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit(handleFormSubmit)} className="space-y-6">
          <Tabs defaultValue="basic" className="w-full">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="basic">{t('providerDialog.tabs.basic')}</TabsTrigger>
              <TabsTrigger value="advanced">{t('providerDialog.tabs.advanced')}</TabsTrigger>
            </TabsList>

            {/* Basic Tab */}
            <TabsContent value="basic" className="space-y-4 mt-4">
              {/* Name */}
              <div className="space-y-2">
                <Label htmlFor="name" className="flex items-center gap-2">
                  {t('providerDialog.fields.name.label')} <span className="text-destructive">{t('providerDialog.requiredField')}</span>
                </Label>
                <Input
                  id="name"
                  placeholder={t('providerDialog.fields.name.placeholder')}
                  {...register('name', {
                    required: t('providerDialog.fields.name.required'),
                  })}
                  className={cn(errors.name && 'border-destructive')}
                />
                {errors.name && (
                  <p className="text-sm text-destructive">{errors.name.message}</p>
                )}
              </div>

              {/* Domain */}
              <div className="space-y-2">
                <Label htmlFor="domain" className="flex items-center gap-2">
                  {t('providerDialog.fields.domain.label')} <span className="text-destructive">{t('providerDialog.requiredField')}</span>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Info className="h-4 w-4 text-muted-foreground cursor-help" />
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>{t('providerDialog.fields.domain.tooltip')}</p>
                    </TooltipContent>
                  </Tooltip>
                </Label>
                <Input
                  id="domain"
                  placeholder={t('providerDialog.fields.domain.placeholder')}
                  {...register('domain', {
                    required: t('providerDialog.fields.domain.required'),
                    pattern: {
                      value: /^https?:\/\/.+/,
                      message: t('providerDialog.fields.domain.invalidFormat'),
                    },
                  })}
                  className={cn(errors.domain && 'border-destructive')}
                />
                {errors.domain && (
                  <p className="text-sm text-destructive">{errors.domain.message}</p>
                )}
              </div>

              {/* WAF Bypass */}
              <div className="flex items-center justify-between space-x-2 rounded-lg border p-4">
                <div className="space-y-0.5">
                  <Label htmlFor="needs_waf_bypass" className="text-base">
                    {t('providerDialog.fields.needsWafBypass.label')}
                  </Label>
                  <div className="text-sm text-muted-foreground">
                    {t('providerDialog.fields.needsWafBypass.description')}
                  </div>
                </div>
                <Switch
                  id="needs_waf_bypass"
                  checked={needsWafBypass}
                  onCheckedChange={(checked) => setValue('needs_waf_bypass', checked)}
                />
              </div>

              {/* Supports check-in */}
              <div className="flex items-center justify-between space-x-2 rounded-lg border p-4">
                <div className="space-y-0.5">
                  <Label htmlFor="supports_check_in" className="text-base">
                    {t('providerDialog.fields.supportsCheckIn.label')}
                  </Label>
                  <div className="text-sm text-muted-foreground">
                    {t('providerDialog.fields.supportsCheckIn.description')}
                  </div>
                </div>
                <Switch
                  id="supports_check_in"
                  checked={supportsCheckIn}
                  onCheckedChange={(checked) => {
                    setValue('supports_check_in', checked);
                    if (!checked) {
                      setValue('check_in_bugged', false);
                    }
                  }}
                />
              </div>

              {/* Known bug toggle */}
              <div className="flex items-center justify-between space-x-2 rounded-lg border p-4">
                <div className="space-y-0.5">
                  <Label htmlFor="check_in_bugged" className="text-base">
                    {t('providerDialog.fields.checkInBugged.label')}
                  </Label>
                  <div className="text-sm text-muted-foreground">
                    {t('providerDialog.fields.checkInBugged.description')}
                  </div>
                </div>
                <Switch
                  id="check_in_bugged"
                  checked={checkInBugged}
                  disabled={!supportsCheckIn}
                  onCheckedChange={(checked) => setValue('check_in_bugged', checked)}
                />
              </div>
            </TabsContent>

            {/* Advanced Tab */}
            <TabsContent value="advanced" className="space-y-4 mt-4">
              <div className="rounded-lg border border-border/50 bg-muted/30 p-4 text-sm text-muted-foreground mb-4">
                <p className="font-medium text-foreground mb-2">💡 {t('providerDialog.advancedNote.title', '默认值说明')}</p>
                <p>{t('providerDialog.advancedNote.description', '以下配置项都是可选的，使用new-api标准默认值。如果你的中转站遵循new-api标准，可以不填写。')}</p>
              </div>

              {/* Login Path */}
              <div className="space-y-2">
                <Label htmlFor="login_path" className="flex items-center gap-2">
                  {t('providerDialog.fields.loginPath.label')}
                  <span className="text-xs text-muted-foreground">({t('common.default', '默认')}: {t('providerDialog.fields.loginPath.placeholder')})</span>
                </Label>
                <Input
                  id="login_path"
                  placeholder={t('providerDialog.fields.loginPath.placeholder')}
                  {...register('login_path')}
                />
              </div>

              {/* Sign In Path */}
              <div className="space-y-2">
                <Label htmlFor="sign_in_path" className="flex items-center gap-2">
                  {t('providerDialog.fields.signInPath.label')}
                  <span className="text-xs text-muted-foreground">({t('common.default', '默认')}: {t('providerDialog.fields.signInPath.placeholder')})</span>
                </Label>
                <Input
                  id="sign_in_path"
                  placeholder={t('providerDialog.fields.signInPath.placeholder')}
                  {...register('sign_in_path')}
                />
              </div>

              {/* User Info Path */}
              <div className="space-y-2">
                <Label htmlFor="user_info_path" className="flex items-center gap-2">
                  {t('providerDialog.fields.userInfoPath.label')}
                  <span className="text-xs text-muted-foreground">({t('common.default', '默认')}: {t('providerDialog.fields.userInfoPath.placeholder')})</span>
                </Label>
                <Input
                  id="user_info_path"
                  placeholder={t('providerDialog.fields.userInfoPath.placeholder')}
                  {...register('user_info_path')}
                />
              </div>

              {/* Token API Path */}
              <div className="space-y-2">
                <Label htmlFor="token_api_path" className="flex items-center gap-2">
                  {t('providerDialog.fields.tokenApiPath.label')}
                  <span className="text-xs text-muted-foreground">({t('common.default', '默认')}: {t('providerDialog.fields.tokenApiPath.placeholder')})</span>
                </Label>
                <Input
                  id="token_api_path"
                  placeholder={t('providerDialog.fields.tokenApiPath.placeholder')}
                  {...register('token_api_path')}
                />
              </div>

              {/* Models Path */}
              <div className="space-y-2">
                <Label htmlFor="models_path" className="flex items-center gap-2">
                  {t('providerDialog.fields.modelsPath.label')}
                  <span className="text-xs text-muted-foreground">({t('common.default', '默认')}: {t('providerDialog.fields.modelsPath.placeholder')})</span>
                </Label>
                <Input
                  id="models_path"
                  placeholder={t('providerDialog.fields.modelsPath.placeholder')}
                  {...register('models_path')}
                />
              </div>

              {/* API User Key */}
              <div className="space-y-2">
                <Label htmlFor="api_user_key" className="flex items-center gap-2">
                  {t('providerDialog.fields.apiUserKey.label')}
                  <span className="text-xs text-muted-foreground">({t('common.default', '默认')}: {t('providerDialog.fields.apiUserKey.placeholder')})</span>
                </Label>
                <Input
                  id="api_user_key"
                  placeholder={t('providerDialog.fields.apiUserKey.placeholder')}
                  {...register('api_user_key')}
                />
              </div>
            </TabsContent>
          </Tabs>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={isSubmitting}
            >
              {t('providerDialog.buttons.cancel')}
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              {mode === 'create'
                ? (isSubmitting ? t('providerDialog.buttons.creating') : t('providerDialog.buttons.create'))
                : (isSubmitting ? t('providerDialog.buttons.saving') : t('providerDialog.buttons.save'))
              }
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
